use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use rusqlite::{Connection};
use uuid::Uuid;

use crate::store_app::user_management::UserDatabase;

#[derive(Clone)]
pub struct SQLiteUserDatabase{
    db: Arc<Mutex<Connection>>,
}

impl SQLiteUserDatabase {
    pub fn new() -> Self {
        // TODO: param in new for slqite file
        let db_path = "db.sqlite";
        let db = Connection::open(db_path).expect("db.sqlite cannot be found");
        SQLiteUserDatabase {
            db: Arc::new(Mutex::new(db)),
        }
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
