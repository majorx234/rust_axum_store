use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait UserDatabase: {
    async fn create_user(&self, username: &str, password_hash: &str);
}

#[async_trait]
pub trait UserPasswordHasher {
    fn hash_password(&self, password: &str) -> String;
}

#[derive(Clone)]
pub struct UserDbPool{
    hasher: Arc<dyn UserPasswordHasher + Send + Sync>,
    user_database: Arc<dyn UserDatabase + Send + Sync>,
}

impl UserDbPool{
    pub fn new(user_database: Arc<dyn UserDatabase + Send + Sync>, hasher: Arc<dyn UserPasswordHasher + Send + Sync>) -> Self {
        Self {
            hasher,
            user_database
        }
    }

    pub async fn add(&self, username: &str, password: &str) {
        let hash = &self.hasher.hash_password(password);
        self.user_database.create_user(username, hash).await;
    }
}
