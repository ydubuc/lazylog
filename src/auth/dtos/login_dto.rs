use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}
