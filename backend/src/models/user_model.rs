use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

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
