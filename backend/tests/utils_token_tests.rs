use chat_app_api::{
    config::environment::EnvironmentConfig,
    utils::token::{create_jwt_token, decode_jwt_token},
};

#[test]
fn test_create_jwt_token() {
    let user_id = "123".to_string();

    let config = EnvironmentConfig::from_env();

    let secret =
        create_jwt_token(&user_id, &config.jwt_secret).expect("Failed to create JWT token");

    let decoded =
        decode_jwt_token(&secret, &config.jwt_secret).expect("Failed to decode JWT token");

    assert_eq!(decoded.sub.to_string(), user_id.to_string());
}
