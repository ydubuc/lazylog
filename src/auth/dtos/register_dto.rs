use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterDto {
    pub username: String,
    pub email: String,
    pub password: String,
}
