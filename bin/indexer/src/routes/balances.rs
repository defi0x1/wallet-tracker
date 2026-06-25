use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_balances(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<Vec<common::TokenBalance>>, StatusCode> {
    db::balances::get_latest_balances(&state.db, &address)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
