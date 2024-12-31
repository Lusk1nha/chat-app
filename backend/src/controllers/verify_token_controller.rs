use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::api_state::ApiState,
    models::auth_model::VerifyTokenResponse,
    repositories::session_repository::SessionRepository,
    services::session_service::SessionService,
    utils::{errors::ErrorResponse, token::decode_jwt_token},
};

pub async fn verify_token_controller(
    token: String,
    secret: String,
    state: Arc<ApiState>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let claims = decode_jwt_token(&token, &secret).map_err(|e| ErrorResponse {
        message: e.to_string(),
        status_code: StatusCode::UNAUTHORIZED,
    })?;

    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo, state.environment.jwt_secret.clone());

    let session = session_service
        .find_session_by_token(&token)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if session.is_none() {
        return Ok(Json(VerifyTokenResponse {
            valid: false,
            decoded: claims,
            error: Some("Session not found or revoked".to_string()),
        }));
    }

    Ok(Json(VerifyTokenResponse {
        valid: true,
        decoded: claims,
        error: None,
    }))
}
