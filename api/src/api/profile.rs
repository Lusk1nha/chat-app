use std::sync::Arc;

use crate::{
    config::api_state::ApiState,
    controllers::profile_controllers::{
        create_profile_controller, find_profile_controller, update_profile_controller,
    },
    models::profile_model::CreateProfileRequest,
    utils::errors::ErrorResponse,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::CookieJar;

pub async fn find_profile_by_user_id_route(
    Path(user_id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse, ErrorResponse> {
    println!("user_id: {}", user_id);
    let user_id = user_id.clone();

    find_profile_controller::find_profile_controller(user_id, state).await
}

pub async fn create_profile_route(
    jar: CookieJar,
    State(state): State<Arc<ApiState>>,
    Json(body): Json<CreateProfileRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let refresh_token = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_default();

    let display_name = body.display_name.clone();
    let bio = body.bio.clone();
    let avatar_url = body.avatar_url.clone();

    create_profile_controller::create_profile_controller(
        refresh_token,
        display_name,
        bio,
        avatar_url,
        state,
    )
    .await
}

pub async fn update_profile_route(
    jar: CookieJar,
    State(state): State<Arc<ApiState>>,
    Json(body): Json<CreateProfileRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let refresh_token = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_default();

    let display_name = body.display_name.clone();
    let bio = body.bio.clone();
    let avatar_url = body.avatar_url.clone();

    update_profile_controller::update_profile_controller(
        refresh_token,
        display_name,
        bio,
        avatar_url,
        state,
    )
    .await
}
