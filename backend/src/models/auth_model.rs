use serde::{Deserialize, Serialize};

use crate::utils::token::Claims;

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize)]
pub struct SignUpResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]

pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyTokenResponse {
    pub valid: bool,
    pub decoded: Claims,
    pub error: Option<String>,
}
