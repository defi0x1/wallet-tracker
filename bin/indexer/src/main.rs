mod routes;
mod state;
mod worker;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tokio::sync::watch;
use tracing::info;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "wallet-tracker=info".into()),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL")?;
    let rpc_url = std::env::var("SOLANA_RPC_URL")?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let state = AppState { db: db.clone() };

    // shutdown channel
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // spawn background worker with shutdown signal
    let worker_handle = tokio::spawn(worker::start(db, rpc_url, shutdown_rx));

    // Build router
    let app = Router::new()
        .route("/wallets", post(routes::wallets::add_wallet))
        .route("/wallets", get(routes::wallets::list_wallets))
        .route(
            "/wallets/{address}/balances",
            get(routes::balances::get_balances),
        )
        .route(
            "/wallets/{address}/transactions",
            get(routes::wallets::get_transactions),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on port 3000");

    // graceful shutdown on Ctrl+C
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
            info!("Shutting down server...");
        })
        .await?;

    // stop background worker
    let _ = shutdown_tx.send(true);
    if tokio::time::timeout(tokio::time::Duration::from_secs(10), worker_handle)
        .await
        .is_err()
    {
        tracing::warn!("Worker did not finish in time, forcing exit");
    }

    info!("Shutdown complete");

    Ok(())
}
