use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::api_state::ApiState,
    models::auth_model::SignUpResponse,
    repositories::{session_repository::SessionRepository, user_repository::UserRepository},
    services::{session_service::SessionService, user_service::UserService},
    utils::{
        errors::ErrorResponse,
        session::{generate_jwt_token, ACCESS_EXPIRES_AT, REFRESH_EXPIRES_AT},
    },
};

pub async fn signup_controller(
    email: String,
    password: String,
    state: Arc<ApiState>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let user_repo = UserRepository::new(state.db.clone());
    let user_service = UserService::new(user_repo);

    let user = user_service
        .create_user(email.as_str(), password.as_str())
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo, state.environment.jwt_secret.clone());

    let access_expires_at = chrono::Utc::now() + chrono::Duration::minutes(ACCESS_EXPIRES_AT);
    let access_token =
        generate_jwt_token(&user.id, &state.environment.jwt_secret, access_expires_at).map_err(
            |e| ErrorResponse {
                message: e.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        )?;

    let refresh_expires_at = chrono::Utc::now() + chrono::Duration::days(REFRESH_EXPIRES_AT);
    let refresh_token =
        generate_jwt_token(&user.id, &state.environment.jwt_secret, refresh_expires_at).map_err(
            |e| ErrorResponse {
                message: e.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        )?;

    session_service
        .create_session(
            &user.id,
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

    Ok((
        StatusCode::CREATED,
        Json(SignUpResponse {
            access_token,
            message: "User created successfully".to_string(),
        }),
    ))
}
