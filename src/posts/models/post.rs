use serde::{Deserialize, Serialize};

pub static POST_SORTABLE_FIELDS: [&str; 2] = ["created_at", "updated_at"];

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
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
