use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    config::api_state::ApiState,
    models::auth_model::LogoutResponse,
    repositories::session_repository::SessionRepository,
    services::session_service::SessionService,
    utils::{cookie::clear_refresh_token_cookie, errors::ErrorResponse, token::decode_jwt_token},
};

pub async fn logout_controller(
    refresh_token: String,
    state: Arc<ApiState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ErrorResponse> {
    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo, state.environment.jwt_secret.clone());

    let secret = state.environment.jwt_secret.clone();

    let claims = decode_jwt_token(&refresh_token, &secret).map_err(|e| ErrorResponse {
        message: e.to_string(),
        status_code: StatusCode::UNAUTHORIZED,
    })?;

    let user_id = claims.sub;

    let session = session_service
        .find_session_by_user_and_refresh_token(&user_id, &refresh_token)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if session.is_none() {
        return Err(ErrorResponse {
            message: "Session not found or revoked".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        });
    }

    session_service
        .delete_session_by_refresh_token(&user_id, &refresh_token)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let jar = clear_refresh_token_cookie(jar);

    let body = LogoutResponse {
        message: "Successfully logged out".to_string(),
    };

    Ok((StatusCode::OK, jar, Json(body)))
}
