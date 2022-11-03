use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: String,
}
