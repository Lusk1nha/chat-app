use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::api_state::ApiState, models::profile_model::ProfileFindResponse,
    repositories::profile_repository::ProfileRepository, services::profile_service::ProfileService,
    utils::errors::ErrorResponse,
};

pub async fn find_profile_by_id_controller(
    id: String,
    state: Arc<ApiState>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let profile_repository = ProfileRepository::new(state.db.clone());
    let profile_service = ProfileService::new(profile_repository);

    let profile = profile_service
        .find_profile_by_id(id)
        .await
        .map_err(|e| ErrorResponse {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    if profile.is_none() {
        return Err(ErrorResponse {
            message: "Profile not found with this id".to_string(),
            status_code: StatusCode::NOT_FOUND,
        });
    }

    let profile = profile.unwrap();

    let body = ProfileFindResponse { profile };

    Ok((StatusCode::OK, Json(body)))
}
