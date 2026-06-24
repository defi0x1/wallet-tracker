use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// Wallet
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Wallet {
    pub address: String,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_synced_at: Option<DateTime<Utc>>,
}

// TokenBalance
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TokenBalance {
    pub id: i32,
    pub wallet_address: String,
    pub mint: String,
    pub amount_raw: i64,
    pub amount: Decimal,
    pub price_usd: Option<Decimal>,
    pub value_usd: Option<Decimal>,
    pub slot: Option<i64>,
    pub snapshot_at: DateTime<Utc>,
}

// Transaction
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub signature: String,
    pub wallet_address: String,
    pub slot: i64,
    pub block_time: DateTime<Utc>,
    pub tx_type: Option<String>,
    pub status: String,
    pub fee_lamports: Option<i64>,
    pub raw_meta: Option<serde_json::Value>,
}

// TransferDirection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum TransferDirection {
    In,
    Out,
}

// TokenTransfer

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TokenTransfer {
    pub id: i32,
    pub signature: String,
    pub wallet_address: String,
    pub mint: String,
    pub direction: TransferDirection,
    pub amount_raw: i64,
    pub amount: Option<Decimal>,
    pub price_usd: Option<Decimal>,
    pub value_usd: Option<Decimal>,
}
