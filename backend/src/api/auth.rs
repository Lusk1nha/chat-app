use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::api_state::ApiState,
    models::{
        auth_model::{
            LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, SignUpRequest,
        },
        user_model::User,
    },
    repositories::{session_repository::SessionRepository, user_repository::UserRepository},
    services::{session_service::SessionService, user_service::UserService},
    utils::{
        errors::ErrorResponse,
        session::{
            create_access_expires_at, create_refresh_expires_at, generate_access_token,
            generate_refresh_token,
        },
        token::create_jwt_token,
        validation::{is_valid_email, is_valid_password},
    },
};

pub async fn signup_route(
    State(state): State<Arc<ApiState>>,
    Json(body): Json<SignUpRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let email = body.email.clone();
    let password = body.password.clone();

    if !is_valid_email(email.as_str()) {
        return Err(ErrorResponse {
            message: "Invalid email".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        });
    }

    if !is_valid_password(password.as_str()) {
        return Err(ErrorResponse {
            message: "Invalid password".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        });
    }

    let user_repo = UserRepository::new(state.db.clone());
    let user_service = UserService::new(user_repo);

    let user_result = user_service
        .create_user(email.as_str(), password.as_str())
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo, state.environment.jwt_secret.clone());

    let session_result = session_service
        .create_session(&user_result.id)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(user_result))
}

pub async fn login_route(
    State(state): State<Arc<ApiState>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let email = body.email.clone();
    let password = body.password.clone();

    let secret = state.environment.jwt_secret.clone();

    if !is_valid_email(email.as_str()) {
        return Err(ErrorResponse {
            message: "Invalid email".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        });
    }

    if !is_valid_password(password.as_str()) {
        return Err(ErrorResponse {
            message: "Invalid password".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        });
    }

    let user_repo = UserRepository::new(state.db.clone());
    let user_service = UserService::new(user_repo);

    let user_result = user_service
        .find_user_by_email(email.as_str())
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if user_result.is_none() {
        return Err(ErrorResponse {
            message: "User not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        });
    }

    let user: User = user_result.unwrap();

    if !user.verify_password(password.as_str()).unwrap() {
        return Err(ErrorResponse {
            message: "Invalid password".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo, state.environment.jwt_secret.clone());

    let new_expires_access_at = create_access_expires_at();
    let new_expires_refresh_at = create_refresh_expires_at();

    let new_access_token =
        generate_access_token(&user.id, &secret).expect("Failed to generate access token");
    let new_refresh_token =
        generate_refresh_token(&user.id, &secret).expect("Failed to generate refresh token");

    Ok(Json(LoginResponse {
        access_token: new_access_token,
        message: "Login successful".to_string(),
    }))
}

pub async fn refresh_token_route(
    State(state): State<Arc<ApiState>>,
    Json(body): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let refresh_token = body.refresh_token.clone();
    let secret = state.environment.jwt_secret.clone();

    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo, state.environment.jwt_secret.clone());

    let session = session_service
        .find_session_by_refresh_token(&refresh_token)
        .await
        .map_err(|e| ErrorResponse {
            message: "Invalid refresh token, error: ".to_string() + e.to_string().as_str(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if session.is_none() {
        return Err(ErrorResponse {
            message: "Invalid refresh token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let session = session.unwrap();

    let new_expires_access_at = create_access_expires_at();
    let new_expires_refresh_at = create_refresh_expires_at();

    let new_access_token =
        generate_access_token(&session.user_id, &secret).expect("Failed to generate access token");
    let new_refresh_token = generate_refresh_token(&session.user_id, &secret)
        .expect("Failed to generate refresh token");

    session_service
        .update_session(
            &session.id,
            &new_access_token,
            &new_refresh_token,
            &new_expires_access_at,
            &new_expires_refresh_at,
        )
        .await
        .map_err(|e| ErrorResponse {
            message: "Failed to update session, error: ".to_string() + e.to_string().as_str(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(RefreshTokenResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
    }))
}
