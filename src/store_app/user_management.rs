use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait UserDatabase: Send + Sync {
    async fn create_user(&self, username: &str, password_hash: &str);
}

pub trait UserPasswordHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> &str;
}

#[derive(Clone)]
pub struct UserPool{
    hasher: Arc<dyn UserPasswordHasher>,
    user_database: Arc<dyn UserDatabase>,
}

impl UserPool{
    pub fn new(user_database: Arc<dyn UserDatabase>, hasher: Arc<dyn UserPasswordHasher>) -> Self {
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
