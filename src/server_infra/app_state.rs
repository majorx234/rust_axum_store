use crate::adapters::crypto::ArgonPasswordHasher;
use crate::adapters::sqlite::SQLiteUserDatabase;
use crate::store_app::user_management::UserDbPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerElements {
    pub user_db_pool: UserDbPool,
}
impl ServerElements {
    pub fn new() -> ServerElements {
//        let db_path = "db.sqlite";
        //        let db = Connection::open(db_path).expect("db.sqlite cannot be found");
        let db = Arc::new(SQLiteUserDatabase::new());
        let hasher = Arc::new(ArgonPasswordHasher::new());
        let user_db_pool = UserDbPool::new(db, hasher);

        ServerElements { user_db_pool }
    }
}
impl Default for ServerElements {
    fn default() -> Self {
        Self::new()
    }
}
