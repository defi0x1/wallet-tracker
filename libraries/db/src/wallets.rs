use common::Wallet;
use sqlx::{Error, PgPool};

pub async fn insert_wallet(
    pool: &PgPool,
    address: &str,
    label: Option<&str>,
) -> Result<Wallet, Error> {
    sqlx::query_as!(
        Wallet,
        "INSERT INTO wallets (address, label)
         VALUES ($1, $2)
         ON CONFLICT (address) DO NOTHING
         RETURNING *",
        address,
        label
    )
    .fetch_one(pool)
    .await
}

pub async fn get_all_wallets(pool: &PgPool) -> Result<Vec<Wallet>, Error> {
    sqlx::query_as!(Wallet, "SELECT * FROM wallets")
        .fetch_all(pool)
        .await
}

pub async fn update_last_synced(pool: &PgPool, address: &str) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE wallets SET last_synced_at = now() WHERE address = $1",
        address
    )
    .execute(pool)
    .await?;
    Ok(())
}
