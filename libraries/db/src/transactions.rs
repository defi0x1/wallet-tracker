use common::{TokenTransfer, Transaction};
use sqlx::{Error, PgPool};

pub async fn insert_transaction(pool: &PgPool, tx: &Transaction) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO transactions 
            (signature, wallet_address, slot, block_time, tx_type, status, fee_lamports, raw_meta)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (signature) DO NOTHING",
        tx.signature,
        tx.wallet_address,
        tx.slot,
        tx.block_time,
        tx.tx_type,
        tx.status,
        tx.fee_lamports,
        tx.raw_meta
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_transfers(pool: &PgPool, transfers: &[TokenTransfer]) -> Result<(), Error> {
    for transfer in transfers {
        sqlx::query!(
            "INSERT INTO token_transfers
                (signature, wallet_address, mint, direction, amount_raw, amount, price_usd, value_usd)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT DO NOTHING",
            transfer.signature,
            transfer.wallet_address,
            transfer.mint,
            transfer.direction as _,
            transfer.amount_raw,
            transfer.amount,
            transfer.price_usd,
            transfer.value_usd
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn get_transactions(
    pool: &PgPool,
    wallet_address: &str,
    limit: i64,
) -> Result<Vec<Transaction>, Error> {
    sqlx::query_as!(
        Transaction,
        "SELECT * FROM transactions
         WHERE wallet_address = $1
         ORDER BY block_time DESC
         LIMIT $2",
        wallet_address,
        limit
    )
    .fetch_all(pool)
    .await
}
