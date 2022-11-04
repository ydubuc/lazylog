use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "title must be between 1 and 255 characters."
    ))]
    pub title: String,
    #[validate(length(
        min = 1,
        max = 65535,
        message = "content must be between 1 and 65535 characters."
    ))]
    pub content: Option<String>,
}
