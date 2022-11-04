use serde::Deserialize;
use validator::Validate;

// static RE_TWO_CHARS: Regex = Regex::new(r"[a-z]{2}$").unwrap();

#[derive(Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(length(
        min = 3,
        max = 24,
        message = "username must be between 3 and 24 characters."
    ))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}
