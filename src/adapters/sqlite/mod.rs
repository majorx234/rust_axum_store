use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use rusqlite::{Connection};
use uuid::Uuid;

use crate::{config::Config, store_app::user_management::UserDatabase};

#[derive(Clone)]
pub struct SQLiteUserDatabase{
    db: Arc<Mutex<Connection>>,
}

impl SQLiteUserDatabase {
    pub fn new() -> Self {
        let config = Config::new();
        let db_path = config.get_sqlite_db_file_path();
        let db = Connection::open(db_path).unwrap_or_else(|_|panic!("{} cannot be found", db_path));
        SQLiteUserDatabase {
            db: Arc::new(Mutex::new(db)),
        }
    }
}
impl Default for SQLiteUserDatabase{
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserDatabase for SQLiteUserDatabase{
    async fn create_user(&self, username: &str, password_hash: &str){
        let uuid = Uuid::new_v4();
        let db = self.db.lock().expect("lock db");
        db.prepare(&format!("INSERT INTO users (id, username, password_hash) VALUES ({}, {}, {})",
            uuid,
            username,
            password_hash)).expect("cannot add user into db");
    }
}
