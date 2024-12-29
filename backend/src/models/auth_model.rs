use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]

pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub maintain_session: Option<bool>,
}
