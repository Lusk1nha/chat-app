use sqlx::{MySql, Pool};

use crate::models::user_model::{ProtectedUser, User};

pub struct UserRepository {
    pub pool: Pool<MySql>,
}

impl UserRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, last_login, is_active, created_at, updated_at FROM users WHERE email = ?",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn create_user(
        &self,
        id: String,
        email: &str,
        password_hash: &str,
    ) -> Result<ProtectedUser, sqlx::Error> {
        let response =
            sqlx::query("INSERT INTO users (id,  email, password_hash) VALUES (?, ?, ?)")
                .bind(id)
                .bind(email)
                .bind(password_hash)
                .execute(&self.pool)
                .await?;

        let user = sqlx::query_as::<_, ProtectedUser>(
            "SELECT id, email, last_login, is_active, created_at, updated_at FROM users WHERE id = ?",
        ).bind(response.last_insert_id()).fetch_one(&self.pool).await?;

        Ok(user)
    }
}
