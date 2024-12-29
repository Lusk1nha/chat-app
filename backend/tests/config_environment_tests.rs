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

    let config = EnvironmentConfig::from_env();

    assert_eq!(
        config.database_url,
        "mysql://user:password@localhost/db_name"
    );
    assert_eq!(config.environment, "production");

    env::remove_var("DATABASE_URL");
    env::remove_var("RUST_ENV");
}
