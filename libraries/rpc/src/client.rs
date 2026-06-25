use crate::error::RpcError;
use crate::parser::{parse_transaction, TokenAccountInfo};
use common::{TokenTransfer, Transaction};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use spl_token::ID as SPL_TOKEN_PROGRAM_ID;
use std::str::FromStr;

pub fn create_client(rpc_url: &str) -> RpcClient {
    RpcClient::new(rpc_url.to_string())
}

// currently, this function only returns token accounts for the SPL Token program
// for token2022 we would need to add later.
pub fn get_token_accounts(
    client: &RpcClient,
    wallet_address: &str,
) -> Result<Vec<TokenAccountInfo>, RpcError> {
    let pubkey =
        Pubkey::from_str(wallet_address).map_err(|e| RpcError::InvalidPubkey(e.to_string()))?;

    let accounts = client.get_token_accounts_by_owner(
        &pubkey,
        TokenAccountsFilter::ProgramId(SPL_TOKEN_PROGRAM_ID),
    )?;

    let mut result = Vec::new();
    for keyed_account in accounts {
        if let solana_account_decoder::UiAccountData::Json(parsed) = keyed_account.account.data {
            let info = &parsed.parsed["info"];
            let mint = info["mint"]
                .as_str()
                .ok_or_else(|| RpcError::Parse("missing mint".into()))?
                .to_string();
            let amount_raw: u64 = info["tokenAmount"]["amount"]
                .as_str()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0);
            let decimals: u8 = info["tokenAmount"]["decimals"].as_u64().unwrap_or(0) as u8;

            result.push(TokenAccountInfo {
                mint,
                amount_raw,
                decimals,
            });
        }
    }
    Ok(result)
}

pub fn get_recent_signatures(
    client: &RpcClient,
    wallet_address: &str,
    limit: usize,
) -> Result<Vec<String>, RpcError> {
    let pubkey =
        Pubkey::from_str(wallet_address).map_err(|e| RpcError::InvalidPubkey(e.to_string()))?;

    let config = solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config {
        limit: Some(limit),
        ..Default::default()
    };

    let sigs = client.get_signatures_for_address_with_config(&pubkey, config)?;
    Ok(sigs.into_iter().map(|s| s.signature).collect())
}

pub fn get_transaction_detail(
    client: &RpcClient,
    signature: &str,
    wallet_address: &str,
) -> Result<Option<(Transaction, Vec<TokenTransfer>)>, RpcError> {
    let sig = Signature::from_str(signature).map_err(|e| RpcError::InvalidPubkey(e.to_string()))?;

    let tx = client.get_transaction(&sig, UiTransactionEncoding::JsonParsed)?;
    Ok(parse_transaction(signature, wallet_address, tx))
}
