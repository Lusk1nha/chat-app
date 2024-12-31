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
            "SELECT id, access_token, refresh_token, access_token_expires_at, refresh_token_expires_at, created_at, updated_at FROM sessions WHERE user_id = ? AND expires_at > NOW()",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn find_session_by_refresh_token(
        &self,
        refresh_token: &String,
    ) -> Result<Option<Session>, sqlx::Error> {
        let session = sqlx::query_as::<_, Session>(
            "SELECT id, user_id, access_token, refresh_token, access_token_expires_at, refresh_token_expires_at, created_at, updated_at FROM sessions WHERE refresh_token = ? AND refresh_token_expires_at > NOW()",
        )
        .bind(refresh_token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn find_session_by_refresh_or_access_token(
        &self,
        token: &String,
    ) -> Result<Option<Session>, sqlx::Error> {
        let session = sqlx::query_as::<_, Session>(
            "SELECT id, user_id, access_token, refresh_token, access_token_expires_at, refresh_token_expires_at, created_at, updated_at FROM sessions WHERE access_token = ? OR refresh_token = ?",
        )
        .bind(token)
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn create_user_session(
        &self,
        id: String,
        user_id: &String,
        access_token: &String,
        refresh_token: &String,
        access_token_expires_at: &DateTime<chrono::Utc>,
        refresh_token_expires_at: &DateTime<chrono::Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO sessions (id, user_id, access_token, refresh_token, access_token_expires_at, refresh_token_expires_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(user_id)
        .bind(access_token)
        .bind(refresh_token)
        .bind(access_token_expires_at)
        .bind(refresh_token_expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_session(
        &self,
        id: &String,
        access_token: &String,
        refresh_token: &String,
        access_token_expires_at: &DateTime<chrono::Utc>,
        refresh_token_expires_at: &DateTime<chrono::Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE sessions SET access_token = ?, refresh_token = ?, access_token_expires_at = ?, refresh_token_expires_at = ? WHERE id = ?",
        )
        .bind(access_token)
        .bind(refresh_token)
        .bind(access_token_expires_at)
        .bind(refresh_token_expires_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_session(&self, refresh_token: &String) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sessions WHERE refresh_token = ?")
            .bind(refresh_token)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
