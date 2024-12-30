use super::token::create_jwt_token;

const REFRESH_EXPIRES_AT: i64 = 7;
const ACCESS_EXPIRES_AT: i64 = 15;

pub fn generate_refresh_token(
    user_id: &String,
    secret: &String,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expires_at = create_refresh_expires_at();
    create_jwt_token(user_id, secret, expires_at.timestamp() as usize)
}

pub fn generate_access_token(
    user_id: &String,
    secret: &String,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expires_at = create_access_expires_at();
    create_jwt_token(user_id, secret, expires_at.timestamp() as usize)
}

pub fn create_refresh_expires_at() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now() + chrono::Duration::days(REFRESH_EXPIRES_AT)
}

pub fn create_access_expires_at() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now() + chrono::Duration::minutes(ACCESS_EXPIRES_AT)
}
