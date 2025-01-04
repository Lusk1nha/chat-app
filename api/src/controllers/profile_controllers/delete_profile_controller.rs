use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    config::api_state::ApiState, repositories::profile_repository::ProfileRepository,
    services::profile_service::ProfileService, utils::errors::ErrorResponse,
};

pub async fn delete_profile_controller(
    user_id: String,
    state: Arc<ApiState>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let profile_repository = ProfileRepository::new(state.db.clone());
    let profile_service = ProfileService::new(profile_repository);

    let profile = profile_service
        .find_profile_by_user_id(user_id)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if profile.is_none() {
        return Err(ErrorResponse {
            message: "Profile not found with this user id".to_string(),
            status_code: StatusCode::NOT_FOUND,
        });
    }

    let profile = profile.unwrap();

    profile_service
        .delete_profile(profile.id)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok((StatusCode::OK, ()))
}
