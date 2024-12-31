use chat_app_api::config::environment::EnvironmentConfig;

fn create_config(
    database_url: &str,
    environment: &str,
    port: &u16,
    jwt_secret: &str,
) -> EnvironmentConfig {
    EnvironmentConfig {
        database_url: database_url.to_string(),
        environment: environment.to_string(),
        port: port.clone(),
        jwt_secret: jwt_secret.to_string(),
    }
}

#[test]
fn it_returns_error_when_environment_is_not_set() {
    let config = create_config("", "", &3000, "123");
    let result = chat_app_api::config::bootstrap::validate_config(&config);
    assert_eq!(result, Err("Environment is not set".to_string()));
}

#[test]
fn it_returns_error_when_database_url_is_not_set() {
    let config = create_config("", "production", &3000, "123");
    let result = chat_app_api::config::bootstrap::validate_config(&config);
    assert_eq!(result, Err("Database URL is not set".to_string()));
}

#[test]
fn it_returns_error_when_port_is_not_set() {
    let config = EnvironmentConfig {
        database_url: "mysql://user:password@localhost/db".to_string(),
        environment: "production".to_string(),
        port: 0,
        jwt_secret: "123".to_string(),
    };

    let result = chat_app_api::config::bootstrap::validate_config(&config);
    assert_eq!(result, Err("Port is not set".to_string()));
}

#[test]
fn it_passes_with_valid_config() {
    let config = create_config(
        "mysql://user:password@localhost/db",
        "production",
        &3000,
        "123",
    );
    let result = chat_app_api::config::bootstrap::validate_config(&config);
    assert!(result.is_ok());
}

#[test]
fn it_returns_error_when_jwt_secret_is_not_set() {
    let config = EnvironmentConfig {
        database_url: "mysql://user:password@localhost/db".to_string(),
        environment: "production".to_string(),
        port: 3000,
        jwt_secret: "".to_string(),
    };

    let result = chat_app_api::config::bootstrap::validate_config(&config);
    assert_eq!(result, Err("JWT Secret is not set".to_string()));
}
