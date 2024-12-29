use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub environment: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let environment = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());

        Self {
            database_url,
            environment,
        }
    }
}
