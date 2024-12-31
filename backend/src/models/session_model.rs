use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,

    pub access_token: String,
    pub refresh_token: String,

    pub access_token_expires_at: DateTime<chrono::Utc>,
    pub refresh_token_expires_at: DateTime<chrono::Utc>,

    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub user_id: String,

    pub access_token: String,
    pub refresh_token: String,

    pub access_token_expires_at: DateTime<chrono::Utc>,
    pub refresh_token_expires_at: DateTime<chrono::Utc>,

    pub expires_at: DateTime<chrono::Utc>,
}
