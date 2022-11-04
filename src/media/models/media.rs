use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub static MEDIA_SORTABLE_FIELDS: [&str; 1] = ["created_at"];

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Media {
    pub id: String,
    pub user_id: String,
    pub url: String,
    pub width: u16,
    pub height: u16,
    pub mime_type: String,
    #[sqlx(try_from = "i64")]
    pub created_at: u64,
}
