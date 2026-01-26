use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait UserDatabase: Send + Sync {
    async fn create_user(&self, username: &str);
}

#[derive(Clone)]
pub struct UserPool{
    user_database: Arc<dyn UserDatabase>
}

impl UserPool{
    pub fn new(user_database: Arc<dyn UserDatabase>) -> Self {
        Self {
            user_database
        }
    }
}
