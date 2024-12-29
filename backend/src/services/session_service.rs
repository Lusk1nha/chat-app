use crate::{
    models::session_model::Session, repositories::session_repository::SessionRepository,
    utils::token::create_token,
};

pub struct SessionService {
    pub repository: SessionRepository,
}

impl SessionService {
    pub fn new(repository: SessionRepository) -> SessionService {
        SessionService { repository }
    }

    pub async fn find_user_session(&self, user_id: String) -> Result<Option<Session>, sqlx::Error> {
        self.repository.find_user_valid_session(user_id).await
    }

    pub async fn create_session(&self, user_id: String) -> Result<(), sqlx::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let expires_at = chrono::Utc::now() + chrono::Duration::days(1);

        let token = create_token(user_id.clone());

        self.repository
            .create_user_session(id, user_id, token, expires_at)
            .await
    }
}
