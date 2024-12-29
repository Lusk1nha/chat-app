use chat_app_api::config::database::get_database_pool;
use chat_app_api::config::environment::EnvironmentConfig;

#[tokio::test]
async fn test_success_connect_database() {
    let config = EnvironmentConfig::from_env();

    let db_pool = get_database_pool(&config)
        .await
        .expect("Failed to connect to database");

    assert!(db_pool.acquire().await.is_ok());
}

#[tokio::test]
async fn test_fail_connect_database() {
    let config = EnvironmentConfig {
        database_url: "invalid_url".to_string(),
        environment: "test".to_string(),
        port: 3000,
    };

    let db_pool = get_database_pool(&config).await;

    assert!(db_pool.is_err());
}
