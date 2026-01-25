use serde_json::json;
use thiserror::Error;
use axum::{http, response::{IntoResponse, Response}};
use tokio::task::JoinError;


#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("TokioTask error: {0}")]
    JoinError(#[from] JoinError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        (http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
