use bcrypt::hash;

pub fn hash_password(password: &str, cost: u32) -> Result<String, bcrypt::BcryptError> {
    hash(password, cost)
}
