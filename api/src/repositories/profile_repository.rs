use sqlx::{MySql, Pool};

use crate::models::profile_model::Profile;

pub struct ProfileRepository {
    pub pool: Pool<MySql>,
}

impl ProfileRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    pub async fn find_profile_by_id(&self, id: String) -> Result<Option<Profile>, sqlx::Error> {
        const QUERY: &str =
            "SELECT id, user_id, display_name, bio, avatar_url, created_at, updated_at FROM profiles WHERE id = ?";

        let profile = sqlx::query_as::<_, Profile>(QUERY)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(profile)
    }

    pub async fn find_profile_by_user_id(
        &self,
        user_id: String,
    ) -> Result<Option<Profile>, sqlx::Error> {
        const QUERY: &str =
            "SELECT id, user_id, display_name, bio, avatar_url, created_at, updated_at FROM profiles WHERE user_id = ?";

        let profile = sqlx::query_as::<_, Profile>(QUERY)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(profile)
    }

    pub async fn create_profile(
        &self,
        id: String,
        user_id: String,
        display_name: String,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Profile, sqlx::Error> {
        const QUERY: &str =
            "INSERT INTO profiles (id, user_id, display_name, bio, avatar_url) VALUES (?, ?, ?, ?, ?)";

        sqlx::query(QUERY)
            .bind(&id)
            .bind(&user_id)
            .bind(&display_name)
            .bind(&bio)
            .bind(&avatar_url)
            .execute(&self.pool)
            .await?;

        const GET_PROFILE_QUERY: &str = "SELECT id, user_id, display_name, bio, avatar_url, created_at, updated_at FROM profiles WHERE id = ?";

        let profile = sqlx::query_as::<_, Profile>(GET_PROFILE_QUERY)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(profile)
    }

    pub async fn update_profile(
        &self,
        user_id: String,
        display_name: String,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Profile, sqlx::Error> {
        const QUERY: &str =
            "UPDATE profiles SET display_name = ?, bio = ?, avatar_url = ? WHERE user_id = ?";

        sqlx::query(QUERY)
            .bind(&display_name)
            .bind(&bio)
            .bind(&avatar_url)
            .bind(&user_id)
            .execute(&self.pool)
            .await?;

        const GET_PROFILE_QUERY: &str = "SELECT id, user_id, display_name, bio, avatar_url, created_at, updated_at FROM profiles WHERE user_id = ?";

        let profile = sqlx::query_as::<_, Profile>(GET_PROFILE_QUERY)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(profile)
    }

    pub async fn delete_profile(&self, id: String) -> Result<(), sqlx::Error> {
        const QUERY: &str = "DELETE FROM profiles WHERE id = ?";

        sqlx::query(QUERY).bind(&id).execute(&self.pool).await?;

        Ok(())
    }
}
