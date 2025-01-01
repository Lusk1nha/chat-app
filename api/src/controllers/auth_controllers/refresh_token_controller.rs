use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;

use crate::{
    config::api_state::ApiState,
    models::auth_model::RefreshTokenResponse,
    repositories::session_repository::SessionRepository,
    services::session_service::SessionService,
    utils::{
        cookie::get_refresh_token_cookie,
        errors::ErrorResponse,
        session::{self, ACCESS_EXPIRES_AT, REFRESH_EXPIRES_AT},
    },
};

pub async fn refresh_token_controller(
    refresh_token: String,
    secret: String,
    state: Arc<ApiState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ErrorResponse> {
    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo, state.environment.jwt_secret.clone());

    let session = session_service
        .find_session_by_refresh_token(&refresh_token)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if session.is_none() {
        return Err(ErrorResponse {
            message: "Invalid refresh token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let session = session.unwrap();

    let access_expires_at = chrono::Utc::now() + chrono::Duration::minutes(ACCESS_EXPIRES_AT);
    let access_token = session::generate_jwt_token(&session.user_id, &secret, access_expires_at)
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let refresh_expires_at = chrono::Utc::now() + chrono::Duration::days(REFRESH_EXPIRES_AT);

    let refresh_token = session::generate_jwt_token(&session.user_id, &secret, refresh_expires_at)
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    session_service
        .update_session(
            &session.id,
            &access_token,
            &refresh_token,
            &access_expires_at,
            &refresh_expires_at,
        )
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let jar = get_refresh_token_cookie(refresh_token, jar);

    let body = RefreshTokenResponse {
        access_token,
        message: "Token refreshed successfully".to_string(),
    };

    Ok((StatusCode::OK, jar, Json(body)))
}
