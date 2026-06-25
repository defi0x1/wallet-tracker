use chrono::{DateTime, TimeZone, Utc};
use common::{TokenTransfer, Transaction, TransferDirection};
use rust_decimal::Decimal;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;

pub struct TokenAccountInfo {
    pub mint: String,
    pub amount_raw: u64,
    pub decimals: u8,
}

pub fn parse_transaction(
    signature: &str,
    wallet_address: &str,
    tx: EncodedConfirmedTransactionWithStatusMeta,
) -> Option<(Transaction, Vec<TokenTransfer>)> {
    let meta = tx.transaction.meta.as_ref()?;
    let block_time: DateTime<Utc> = Utc.timestamp_opt(tx.block_time?, 0).single()?;

    let status = if meta.err.is_none() {
        "success"
    } else {
        "failed"
    }
    .to_string();

    let fee_lamports = Some(meta.fee as i64);
    let raw_meta = serde_json::to_value(meta).ok();

    let transaction = Transaction {
        signature: signature.to_string(),
        wallet_address: wallet_address.to_string(),
        slot: tx.slot as i64,
        block_time,
        tx_type: None, // can be enriched later
        status,
        fee_lamports,
        raw_meta,
    };

    let empty = vec![];
    let pre: &[_] = meta
        .pre_token_balances
        .as_ref()
        .map(|v| v.as_slice())
        .unwrap_or(&empty);
    let post: &[_] = meta
        .post_token_balances
        .as_ref()
        .map(|v| v.as_slice())
        .unwrap_or(&empty);

    let mut transfers = Vec::new();

    for post_balance in post {
        let mint = &post_balance.mint;
        let owner = post_balance
            .owner
            .as_ref()
            .map(|o| o.as_str())
            .unwrap_or("");

        if owner != wallet_address {
            continue;
        }

        let post_amount: u64 = post_balance.ui_token_amount.amount.parse().unwrap_or(0);

        let pre_amount: u64 = pre
            .iter()
            .find(|p| {
                p.mint == *mint && p.owner.as_ref().map(|o| o.as_str()) == Some(wallet_address)
            })
            .and_then(|p| p.ui_token_amount.amount.parse().ok())
            .unwrap_or(0);

        if post_amount == pre_amount {
            continue;
        }

        let (direction, amount_raw) = if post_amount > pre_amount {
            (TransferDirection::In, (post_amount - pre_amount) as i64)
        } else {
            (TransferDirection::Out, (pre_amount - post_amount) as i64)
        };

        let decimals = post_balance.ui_token_amount.decimals as u32;
        let amount = Decimal::from(amount_raw) / Decimal::from(10u64.pow(decimals));

        transfers.push(TokenTransfer {
            id: 0, // DB will assign
            signature: signature.to_string(),
            wallet_address: wallet_address.to_string(),
            mint: mint.clone(),
            direction,
            amount_raw,
            amount: Some(amount),
            price_usd: None, // enriched later by price lib
            value_usd: None,
        });
    }

    Some((transaction, transfers))
}
