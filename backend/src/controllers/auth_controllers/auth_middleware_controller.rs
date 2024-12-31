use std::sync::Arc;

use axum::{
    body::Body, extract::Request, http::StatusCode, middleware::Next, response::IntoResponse,
};

use crate::{
    config::api_state::ApiState,
    repositories::session_repository::SessionRepository,
    services::session_service::SessionService,
    utils::{errors::ErrorResponse, token::decode_jwt_token},
};

pub async fn auth_middleware_controller(
    req: Request<Body>,
    next: Next,
    state: Arc<ApiState>,
    token: String,
) -> Result<impl IntoResponse, ErrorResponse> {
    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo, state.environment.jwt_secret.clone());

    let secret = state.environment.jwt_secret.clone();

    let claims = decode_jwt_token(&token, &secret).map_err(|e| ErrorResponse {
        message: e.to_string(),
        status_code: StatusCode::UNAUTHORIZED,
    })?;

    let session = session_service
        .find_access_token(&claims.sub, &token)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if session.is_none() {
        return Err(ErrorResponse {
            message: "Session not found or revoked".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    Ok(next.run(req).await)
}
