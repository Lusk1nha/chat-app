use super::environment::EnvironmentConfig;

pub fn validate_config(config: &EnvironmentConfig) -> Result<(), String> {
    if config.environment.is_empty() {
        return Err("Environment is not set".to_string());
    }

    if config.database_url.is_empty() {
        return Err("Database URL is not set".to_string());
    }

    if config.port == 0 {
        return Err("Port is not set".to_string());
    }

    if config.jwt_secret.is_empty() {
        return Err("JWT Secret is not set".to_string());
    }

    Ok(())
}
