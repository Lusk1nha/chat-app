use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::api_state::ApiState, models::auth_model::LogoutResponse,
    repositories::session_repository::SessionRepository, services::session_service::SessionService,
    utils::errors::ErrorResponse,
};

pub async fn logout_controller(
    refresh_token: String,
    state: Arc<ApiState>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo, state.environment.jwt_secret.clone());

    session_service
        .delete_session_by_refresh_token(&refresh_token)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(LogoutResponse {
        message: "Successfully logged out".to_string(),
    }))
}
