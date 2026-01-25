use axum::{Json, extract::State};
use crate::{server_state::ServerState, error::AppError};

pub async fn hello_handler(State(state): State<ServerState>) -> Result<Json<String>, AppError> {
    let text: Result<String, AppError> = tokio::task::spawn_blocking(move || {
         let db = &state.lock().unwrap().db;
         let mut stmt = db.prepare("SELECT text FROM messages LIMIT 1")?;
         let mut rows = stmt.query([])?;
         if let Some(text) = rows.next()? {
             let text: String = text.get(0)?;
             Ok(text)
         } else{
             Ok("none".to_string())
         }
    }).await?;

    Ok(Json(text.expect("database handler error")))
}
