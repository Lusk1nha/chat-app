use crate::{
    models::session_model::Session, repositories::session_repository::SessionRepository,
    utils::token::create_jwt_token,
};

pub struct SessionService {
    pub repository: SessionRepository,
    pub secret: String,
}

impl SessionService {
    pub fn new(repository: SessionRepository, secret: String) -> SessionService {
        SessionService { repository, secret }
    }

    pub async fn find_session_by_refresh_token(
        &self,
        refresh_token: &String,
    ) -> Result<Option<Session>, sqlx::Error> {
        self.repository
            .find_session_by_refresh_token(refresh_token)
            .await
    }

    pub async fn create_session(&self, user_id: &String) -> Result<(), sqlx::Error> {
        let id = uuid::Uuid::new_v4().to_string();

        let access_token_expires_at = chrono::Utc::now() + chrono::Duration::minutes(15);
        let refresh_token_expires_at = chrono::Utc::now() + chrono::Duration::days(7);

        let access_token = create_jwt_token(
            &user_id,
            &self.secret,
            access_token_expires_at.timestamp() as usize,
        )
        .expect("Failed to create JWT token");
        let refresh_token = create_jwt_token(
            &user_id,
            &self.secret,
            refresh_token_expires_at.timestamp() as usize,
        )
        .expect("Failed to create JWT token");

        self.repository
            .create_user_session(
                id,
                user_id,
                &access_token,
                &refresh_token,
                access_token_expires_at,
                refresh_token_expires_at,
            )
            .await
    }

    pub async fn update_session(
        &self,
        id: &String,
        access_token: &String,
        refresh_token: &String,
        access_token_expires_at: &chrono::DateTime<chrono::Utc>,
        refresh_token_expires_at: &chrono::DateTime<chrono::Utc>,
    ) -> Result<(), sqlx::Error> {
        self.repository
            .update_session(
                id,
                access_token,
                refresh_token,
                access_token_expires_at,
                refresh_token_expires_at,
            )
            .await
    }
}
