use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::hash::verify_hash_password;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: String,

    pub email: String,
    pub password_hash: String,

    pub last_login: Option<DateTime<chrono::Utc>>,
    pub is_active: bool,

    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

impl User {
    pub fn new(
        id: String,
        email: String,
        password_hash: String,
        last_login: Option<DateTime<chrono::Utc>>,
        is_active: bool,
        created_at: DateTime<chrono::Utc>,
        updated_at: DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            email,
            password_hash,
            last_login,
            is_active,
            created_at,
            updated_at,
        }
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify_hash_password(password, &self.password_hash)
    }
}

#[derive(Debug, FromRow, Serialize)]
pub struct ProtectedUser {
    pub id: String,

    pub email: String,
    pub last_login: Option<DateTime<chrono::Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]

pub struct UserUpdate {
    pub email: Option<String>,
    pub password: Option<String>,
    pub last_login: Option<DateTime<chrono::Utc>>,
    pub is_active: Option<bool>,
}
