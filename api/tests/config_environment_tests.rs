use chat_app_api::config::environment::EnvironmentConfig;

use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;

lazy_static! {
    static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
fn test_from_env_with_valid_vars() {
    let _lock = ENV_MUTEX.lock().unwrap();
    env::set_var("DATABASE_URL", "mysql://user:password@localhost/db_name");
    env::set_var("RUST_ENV", "production");
    env::set_var("JWT_SECRET", "secret");

    let config = EnvironmentConfig::from_env();

    assert_eq!(
        config.database_url,
        "mysql://user:password@localhost/db_name"
    );
    assert_eq!(config.environment, "production");
    assert_eq!(config.port, 3000);
    assert_eq!(config.jwt_secret, "secret");

    env::remove_var("DATABASE_URL");
    env::remove_var("RUST_ENV");
    env::remove_var("JWT_SECRET");
}
