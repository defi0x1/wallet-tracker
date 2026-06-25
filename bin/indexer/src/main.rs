mod routes;
mod state;
mod worker;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use state::AppState;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")?;
    let rpc_url = std::env::var("SOLANA_RPC_URL")?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let state = AppState { db: db.clone() };

    // Spawn background worker
    tokio::spawn(worker::start(db, rpc_url));

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
    axum::serve(listener, app).await?;

    Ok(())
}
