use regex::Regex;

const EMAIL_REGEX: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
const PASSWORD_LENGTH: usize = 5;

pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(EMAIL_REGEX).unwrap();
    re.is_match(email)
}

pub fn is_valid_password(password: &str) -> bool {
    password.len() >= PASSWORD_LENGTH
}
