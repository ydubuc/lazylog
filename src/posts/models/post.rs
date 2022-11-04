use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{auth::jwt::models::claims::Claims, posts::dtos::create_post_dto::CreatePostDto};

pub static POST_SORTABLE_FIELDS: [&str; 2] = ["created_at", "updated_at"];

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[sqlx(try_from = "i64")]
    pub updated_at: u64,
    #[sqlx(try_from = "i64")]
    pub created_at: u64,
}

impl Post {
    pub fn new(claims: &Claims, dto: &CreatePostDto) -> Self {
        return Self {
            id: Uuid::new_v4().to_string(),
            user_id: claims.id.to_string(),
            title: dto.title.to_string(),
            content: dto.content.to_owned(),
            updated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
    }
}
