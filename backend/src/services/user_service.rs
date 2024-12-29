use bcrypt::DEFAULT_COST;

use crate::{
    models::user_model::{ProtectedUser, User},
    repositories::user_repository::UserRepository,
    utils::hash::hash_password,
};

pub struct UserService {
    pub repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        self.repository.find_user_by_email(email).await
    }

    pub async fn create_user(
        &self,
        email: &str,
        password_hash: &str,
    ) -> Result<ProtectedUser, sqlx::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let password = match create_hashed_password(password_hash) {
            Ok(password) => password,
            Err(e) => return Err(sqlx::Error::Decode(Box::new(e))),
        };

        self.repository.create_user(id, email, &password).await
    }
}

fn create_hashed_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash_password(password, DEFAULT_COST)
}
