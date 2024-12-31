use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::{
    config::api_state::ApiState,
    controllers::auth_controllers::auth_middleware_controller,
    utils::{errors::ErrorResponse, token::is_valid_jwt_token},
};

pub async fn auth_middleware(
    req: Request<Body>,
    next: Next,
    state: Arc<ApiState>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let authorization = req.headers().get("Authorization");
    let secret = state.environment.jwt_secret.clone();

    match authorization {
        Some(_) => {}
        None => {
            return Err(ErrorResponse {
                message: "Authorization header is missing".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    }

    let token = match authorization.unwrap().to_str() {
        Ok(token_str) => token_str.trim().to_string().replace("Bearer ", ""),
        Err(_) => {
            return Err(ErrorResponse {
                message: "Invalid token format".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };

    if !is_valid_jwt_token(&token, &secret) {
        return Err(ErrorResponse {
            message: "Invalid token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    auth_middleware_controller::auth_middleware_controller(req, next, state, token).await
}
