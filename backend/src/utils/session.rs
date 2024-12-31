use super::token::create_jwt_token;

pub const REFRESH_EXPIRES_AT: i64 = 7;
pub const ACCESS_EXPIRES_AT: i64 = 15;

pub fn generate_jwt_token(
    user_id: &String,
    secret: &String,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expires_at_timestamp = expires_at.timestamp() as usize;
    create_jwt_token(user_id, secret, expires_at_timestamp)
}
