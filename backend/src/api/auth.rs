use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::api_state::ApiState,
    models::auth_model::SignUpRequest,
    repositories::{session_repository::SessionRepository, user_repository::UserRepository},
    services::{session_service::SessionService, user_service::UserService},
    utils::{
        errors::ErrorResponse,
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
        println!("Invalid email: {}", email);
        return Err(ErrorResponse {
            message: "Invalid email".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        });
    }

    if !is_valid_password(password.as_str()) {
        println!("Invalid password: {}", password);
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

    println!("User created: {:?}", user_result);

    let session_repo = SessionRepository::new(state.db.clone());
    let session_service = SessionService::new(session_repo);

    let session_result = session_service
        .create_session(user_result.id)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(session_result))
}
