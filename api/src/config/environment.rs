use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    pub database_url: String,
    pub environment: String,
    pub port: u16,

    pub jwt_secret: String,
}

impl EnvironmentConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let environment = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
        let port = env::var("RUST_PORT_ENV")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        Self {
            database_url,
            environment,
            port,
            jwt_secret,
        }
    }
}
