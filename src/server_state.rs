use std:: sync::Mutex;
use std::sync::Arc;

use rusqlite::{Connection};

#[derive(Clone)]
pub struct ServerState {
    pub db: Arc<Mutex<Connection>>,
}
impl ServerState {
    pub fn new() -> ServerState {
        let db_path = "db.sqlite";
        let db = Connection::open(db_path).expect("db.sqlite cannot be found");

        ServerState {
            db: Arc::new(Mutex::new(db)),
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}
