use axum::{Json, extract::State};
use crate::{ error::AppError, server_infra::app_state::ServerElements};

#[axum::debug_handler]
pub async fn hello_store(State(state): State<ServerElements>) -> Result<Json<String>, AppError> {

    Ok(Json("{\"text\":\"hello store\"}".to_string()))
}
