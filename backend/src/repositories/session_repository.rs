use chrono::DateTime;
use sqlx::{MySql, Pool};

use crate::models::session_model::Session;

pub struct SessionRepository {
    pub pool: Pool<MySql>,
}

impl SessionRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    pub async fn find_user_valid_session(
        &self,
        user_id: String,
    ) -> Result<Option<Session>, sqlx::Error> {
        let session = sqlx::query_as::<_, Session>(
            "SELECT id, token, expires_at, created_at, updated_at FROM sessions WHERE user_id = ? AND expires_at > NOW()",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn create_user_session(
        &self,
        id: String,
        user_id: &String,
        token: &String,
        expires_at: &DateTime<chrono::Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO sessions (id, user_id, token, expires_at) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(user_id)
            .bind(token)
            .bind(expires_at)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
