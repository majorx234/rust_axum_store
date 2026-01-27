use std::sync::Mutex;
use std::sync::Arc;

use rusqlite::{Connection};

pub struct ServerElements {
    pub db: Connection,
}
impl ServerElements {
    pub fn new() -> ServerElements {
        let db_path = "db.sqlite";
        let db = Connection::open(db_path).expect("db.sqlite cannot be found");

        ServerElements { db }
    }
}
impl Default for ServerElements {
    fn default() -> Self {
        Self::new()
    }
}

pub type ServerState = Arc<Mutex<ServerElements>>;
