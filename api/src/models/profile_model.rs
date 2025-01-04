use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]

pub struct Profile {
    pub id: String,
    pub user_id: String,

    #[serde(rename = "displayName")]
    pub display_name: String,
    pub bio: Option<String>,

    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProfileFindResponse {
    pub profile: Profile,
}

#[derive(Debug, Deserialize)]
pub struct CreateProfileRequest {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub bio: Option<String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateProfileResponse {
    pub profile: Profile,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub bio: Option<String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateProfileResponse {
    pub profile: Profile,
}
