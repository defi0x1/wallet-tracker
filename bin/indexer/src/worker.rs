use sqlx::PgPool;
use std::time::Duration;
use tokio::sync::watch;

pub async fn start(db: PgPool, rpc_url: String, mut shutdown_rx: watch::Receiver<bool>) {
    let client = rpc::client::create_client(&rpc_url);

    loop {
        // check shutdown before each cycle
        if *shutdown_rx.borrow() {
            tracing::info!("Worker shutting down");
            break;
        }

        let wallets = match db::wallets::get_all_wallets(&db).await {
            Ok(w) => w,
            Err(e) => {
                tracing::error!("Failed to fetch wallets: {e}");
                tokio::time::sleep(Duration::from_secs(30)).await;
                continue;
            }
        };

        for wallet in wallets {
            if let Err(e) = sync_wallet(&client, &db, &wallet.address).await {
                tracing::error!("Failed to sync wallet {}: {e}", wallet.address);
            }
        }

        // wait 60s but wake up immediately if shutdown signal arrives
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(60)) => {}
            _ = shutdown_rx.changed() => {
                tracing::info!("Worker shutting down during sleep");
                break;
            }
        }
    }
}

async fn sync_wallet(
    client: &solana_client::rpc_client::RpcClient,
    db: &PgPool,
    address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch and save token balances
    let token_accounts = rpc::client::get_token_accounts(client, address)?;
    let now = chrono::Utc::now();

    for account in token_accounts {
        let amount = rust_decimal::Decimal::from(account.amount_raw)
            / rust_decimal::Decimal::from(10u64.pow(account.decimals as u32));

        db::balances::upsert_balance(
            db,
            address,
            &account.mint,
            account.amount_raw as i64,
            amount,
            None,
            None,
            None,
            now,
        )
        .await?;
    }

    // Fetch and save recent transactions
    let signatures = rpc::client::get_recent_signatures(client, address, 20)?;

    for sig in signatures {
        if let Some((tx, transfers)) = rpc::client::get_transaction_detail(client, &sig, address)? {
            db::transactions::insert_transaction(db, &tx).await?;
            db::transactions::insert_transfers(db, &transfers).await?;
        }
    }

    db::wallets::update_last_synced(db, address).await?;
    Ok(())
}
