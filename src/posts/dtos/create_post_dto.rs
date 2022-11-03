use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreatePostDto {
    pub title: String,
    pub content: Option<String>,
}
