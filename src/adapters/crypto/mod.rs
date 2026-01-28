use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::store_app::user_management::UserPasswordHasher;

#[derive(Default)]
pub struct ArgonPasswordHasher {
    hasher: Argon2<'static>,
}

impl ArgonPasswordHasher {
    pub fn new() -> Self {
        let hasher = Argon2::default();
        ArgonPasswordHasher {
            hasher
        }
    }
}

impl UserPasswordHasher for ArgonPasswordHasher {
    // TODO: error handling here
    fn hash_password(&self, password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .hasher
            .hash_password(password.as_bytes(), &salt)
            .expect("password hash error");

        hash.to_string()
    }
}
