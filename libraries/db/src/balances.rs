use chrono::{DateTime, Utc};
use common::TokenBalance;
use rust_decimal::Decimal;
use sqlx::{Error, PgPool};

pub async fn upsert_balance(
    pool: &PgPool,
    wallet_address: &str,
    mint: &str,
    amount_raw: i64,
    amount: Decimal,
    price_usd: Option<Decimal>,
    value_usd: Option<Decimal>,
    slot: Option<i64>,
    snapshot_at: DateTime<Utc>,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO token_balances
            (wallet_address, mint, amount_raw, amount, price_usd, value_usd, slot, snapshot_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (wallet_address, mint, snapshot_at) DO NOTHING",
        wallet_address,
        mint,
        amount_raw,
        amount,
        price_usd,
        value_usd,
        slot,
        snapshot_at
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_latest_balances(
    pool: &PgPool,
    wallet_address: &str,
) -> Result<Vec<TokenBalance>, Error> {
    sqlx::query_as!(
        TokenBalance,
        "SELECT DISTINCT ON (mint) *
         FROM token_balances
         WHERE wallet_address = $1
         ORDER BY mint, snapshot_at DESC",
        wallet_address
    )
    .fetch_all(pool)
    .await
}
