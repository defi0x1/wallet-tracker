use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddWalletRequest {
    pub address: String,
    pub label: Option<String>,
}

pub async fn add_wallet(
    State(state): State<AppState>,
    Json(body): Json<AddWalletRequest>,
) -> Result<Json<common::Wallet>, StatusCode> {
    db::wallets::insert_wallet(&state.db, &body.address, body.label.as_deref())
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn list_wallets(
    State(state): State<AppState>,
) -> Result<Json<Vec<common::Wallet>>, StatusCode> {
    db::wallets::get_all_wallets(&state.db)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_transactions(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<Vec<common::Transaction>>, StatusCode> {
    db::transactions::get_transactions(&state.db, &address, 50)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
