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
        const QUERY: &str = "SELECT id, email, password_hash, last_login, is_active, created_at, updated_at FROM users WHERE email = ?";
        let user = sqlx::query_as::<_, User>(QUERY)
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
        const CREATE_USER_QUERY: &str =
            "INSERT INTO users (id, email, password_hash) VALUES (?, ?, ?)";

        sqlx::query(CREATE_USER_QUERY)
            .bind(&id)
            .bind(&email)
            .bind(&password_hash)
            .execute(&self.pool)
            .await
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        const GET_USER_QUERY: &str = "SELECT id, email, last_login, is_active, created_at, updated_at FROM users WHERE id = ?";

        let user = sqlx::query_as::<_, ProtectedUser>(GET_USER_QUERY)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn update_user_last_login(&self, id: &str) -> Result<(), sqlx::Error> {
        const QUERY: &str = "UPDATE users SET last_login = NOW() WHERE id = ?";

        sqlx::query(QUERY)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(())
    }
}
