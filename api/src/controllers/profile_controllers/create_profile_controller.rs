use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::api_state::ApiState,
    models::profile_model::CreateProfileResponse,
    repositories::profile_repository::ProfileRepository,
    services::profile_service::ProfileService,
    utils::{errors::ErrorResponse, token::decode_jwt_token},
};

pub async fn create_profile_controller(
    user_id: String,
    display_name: String,
    bio: Option<String>,
    avatar_url: Option<String>,
    state: Arc<ApiState>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let profile_repository = ProfileRepository::new(state.db.clone());
    let profile_service = ProfileService::new(profile_repository);

    let profile = profile_service
        .find_profile_by_user_id(user_id.clone())
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if profile.is_some() {
        return Err(ErrorResponse {
            message: "Profile already exists".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        });
    }

    let response = profile_service
        .create_profile(user_id, display_name, bio, avatar_url)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let body = CreateProfileResponse { profile: response };

    Ok((StatusCode::CREATED, Json(body)))
}
