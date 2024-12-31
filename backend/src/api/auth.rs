use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::api_state::ApiState,
    controllers::{
        login_controller::login_controller, logout_controller::logout_controller,
        refresh_token_controller::refresh_token_controller, signup_controller::signup_controller,
        verify_token_controller::verify_token_controller,
    },
    models::auth_model::{
        LoginRequest, LogoutRequest, RefreshTokenRequest, SignUpRequest, VerifyTokenRequest,
    },
    utils::{
        errors::ErrorResponse,
        token::is_valid_jwt_token,
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

    signup_controller(email, password, state).await
}

pub async fn login_route(
    State(state): State<Arc<ApiState>>,
    Json(body): Json<LoginRequest>,
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

    login_controller(email, password, state).await
}

pub async fn logout_route(
    State(state): State<Arc<ApiState>>,
    Json(body): Json<LogoutRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let refresh_token = body.refresh_token.clone();

    if !is_valid_jwt_token(&refresh_token, &state.environment.jwt_secret) {
        return Err(ErrorResponse {
            message: "Invalid refresh token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    logout_controller(refresh_token, state).await
}

pub async fn refresh_token_route(
    State(state): State<Arc<ApiState>>,
    Json(body): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let refresh_token = body.refresh_token.clone();
    let secret = state.environment.jwt_secret.clone();

    if !is_valid_jwt_token(&refresh_token, &secret) {
        return Err(ErrorResponse {
            message: "Invalid refresh token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    refresh_token_controller(refresh_token, secret, state).await
}

pub async fn verify_token_route(
    State(state): State<Arc<ApiState>>,
    Json(body): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let refresh_token = body.token.clone();
    let secret = state.environment.jwt_secret.clone();

    if !is_valid_jwt_token(&refresh_token, &secret) {
        return Err(ErrorResponse {
            message: "Invalid refresh token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    verify_token_controller(refresh_token, secret, state).await
}
