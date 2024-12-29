pub fn create_token(user_id: String) -> String {
    "Token".to_string() + user_id.as_str()
}
