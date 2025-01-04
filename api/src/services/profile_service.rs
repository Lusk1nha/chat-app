use crate::{models::profile_model::Profile, repositories::profile_repository::ProfileRepository};

pub struct ProfileService {
    pub repository: ProfileRepository,
}

impl ProfileService {
    pub fn new(repository: ProfileRepository) -> Self {
        Self { repository }
    }

    pub async fn find_profile_by_id(&self, id: String) -> Result<Option<Profile>, sqlx::Error> {
        self.repository.find_profile_by_id(id).await
    }

    pub async fn find_profile_by_user_id(
        &self,
        user_id: String,
    ) -> Result<Option<Profile>, sqlx::Error> {
        self.repository.find_profile_by_user_id(user_id).await
    }

    pub async fn create_profile(
        &self,
        user_id: String,
        display_name: String,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Profile, sqlx::Error> {
        let id = uuid::Uuid::new_v4().to_string();

        self.repository
            .create_profile(id, user_id, display_name, bio, avatar_url)
            .await
    }

    pub async fn update_profile(
        &self,
        user_id: String,
        display_name: String,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Profile, sqlx::Error> {
        self.repository
            .update_profile(user_id, display_name, bio, avatar_url)
            .await
    }

    pub async fn delete_profile(&self, id: String) -> Result<(), sqlx::Error> {
        self.repository.delete_profile(id).await
    }
}
