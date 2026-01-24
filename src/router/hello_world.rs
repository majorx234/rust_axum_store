use axum::{Json, extract::State, http::Error};
use crate::{server_state::ServerState};

pub async fn hello_handler(State(state): State<ServerState>) -> Result<Json<String>, Error> {
    let state = state;
    //db.clone();

    // TODO do sqlite stuff here
    // let text = tokio::task::spawn_blocking(move || {
        // let db = state.lock().unwrap().db;
        // let mut stmt = db.prepare("SELECT text FROM messages LIMIT 1")?;
        // let mut rows = stmt.query([])?;
        // let text: String = rows.next()?.get(0)?;
    //    Ok(text)
    //})
    //.await?;

    let text = "Hello world".to_string();

    Ok(Json(text))
}
