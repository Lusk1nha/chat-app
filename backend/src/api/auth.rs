use std::sync::Arc;

use crate::{
    config::api_state::ApiState,
    controllers::auth_controllers::{
        login_controller, logout_controller, refresh_token_controller, signup_controller,
        verify_token_controller,
    },
    models::auth_model::{
        LoginRequest, LogoutRequest, RefreshTokenRequest, SignUpRequest, VerifyTokenRequest,
    },
    utils::{
        errors::ErrorResponse,
        token::is_valid_jwt_token,
        validation::{is_valid_confirm_password, is_valid_email, is_valid_password},
    },
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar};

pub async fn signup_route(
    jar: CookieJar,
    State(state): State<Arc<ApiState>>,
    Json(body): Json<SignUpRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let email = body.email.clone();
    let password = body.password.clone();
    let confirm_password = body.confirm_password.clone();

    if email.is_empty() || password.is_empty() || confirm_password.is_empty() {
        return Err(ErrorResponse {
            message: "Email, password, and confirm password are required".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        });
    }

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

    if !is_valid_confirm_password(password.as_str(), confirm_password.as_str()) {
        return Err(ErrorResponse {
            message: "Passwords do not match".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        });
    }

    signup_controller::signup_controller(email, password, state, jar).await
}

pub async fn login_route(
    jar: CookieJar,
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

    login_controller::login_controller(email, password, state, jar).await
}

pub async fn logout_route(
    jar: CookieJar,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let refresh_token = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_default();

    if !is_valid_jwt_token(&refresh_token, &state.environment.jwt_secret) {
        return Err(ErrorResponse {
            message: "Invalid refresh token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    logout_controller::logout_controller(refresh_token, state, jar).await
}

pub async fn refresh_token_route(
    jar: CookieJar,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let refresh_token = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_default();

    println!("refresh_token: {}", refresh_token);

    let secret = state.environment.jwt_secret.clone();

    if !is_valid_jwt_token(&refresh_token, &secret) {
        return Err(ErrorResponse {
            message: "Invalid refresh token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    refresh_token_controller::refresh_token_controller(refresh_token, secret, state, jar).await
}

pub async fn verify_token_route(
    jar: CookieJar,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let refresh_token = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_default();
    let secret = state.environment.jwt_secret.clone();

    if !is_valid_jwt_token(&refresh_token, &secret) {
        return Err(ErrorResponse {
            message: "Invalid refresh token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    verify_token_controller::verify_token_controller(refresh_token, secret, state).await
}
