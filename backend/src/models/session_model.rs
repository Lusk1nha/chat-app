use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,

    pub token: String,
    pub expires_at: DateTime<chrono::Utc>,

    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub user_id: String,
    pub token: String,
    pub expires_at: DateTime<chrono::Utc>,
}
