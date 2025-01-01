use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::api_state::ApiState,
    models::profile_model::UpdateProfileResponse,
    repositories::profile_repository::ProfileRepository,
    services::profile_service::ProfileService,
    utils::{errors::ErrorResponse, token::decode_jwt_token},
};

pub async fn update_profile_controller(
    refresh_token: String,
    display_name: String,
    bio: Option<String>,
    avatar_url: Option<String>,
    state: Arc<ApiState>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let claims = decode_jwt_token(&refresh_token, &state.environment.jwt_secret).map_err(|e| {
        ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        }
    })?;

    let profile_repository = ProfileRepository::new(state.db.clone());
    let profile_service = ProfileService::new(profile_repository);

    let profile = profile_service
        .update_profile(claims.sub, display_name, bio, avatar_url)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let body = UpdateProfileResponse { profile };

    Ok((StatusCode::OK, Json(body)))
}
