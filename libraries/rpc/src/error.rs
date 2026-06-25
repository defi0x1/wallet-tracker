use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("Solana client error: {0}")]
    Client(#[from] solana_client::client_error::ClientError),

    #[error("Invalid pubkey: {0}")]
    InvalidPubkey(String),

    #[error("Parse error: {0}")]
    Parse(String),
}
