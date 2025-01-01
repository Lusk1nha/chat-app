use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::api_state::ApiState,
    models::auth_model::ValidateSessionResponse,
    repositories::session_repository::SessionRepository,
    services::session_service::SessionService,
    utils::{errors::ErrorResponse, token::decode_jwt_token},
};

pub async fn validate_session_controller(
    refresh_token: String,
    state: Arc<ApiState>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let claims = decode_jwt_token(&refresh_token, &state.environment.jwt_secret).map_err(|e| {
        ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        }
    })?;

    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo, state.environment.jwt_secret.clone());

    let session = session_service
        .find_session_by_user_and_refresh_token(&claims.sub, &refresh_token)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if session.is_none() {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(ValidateSessionResponse {
                valid: false,
                message: "Session is invalid".to_string(),
            }),
        ));
    }

    let body = ValidateSessionResponse {
        valid: true,
        message: "Session is valid".to_string(),
    };

    Ok((StatusCode::OK, Json(body)))
}
