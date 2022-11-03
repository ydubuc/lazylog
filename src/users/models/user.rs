use serde::{Deserialize, Serialize};

pub static USER_SORTABLE_FIELDS: [&str; 2] = ["created_at", "updated_at"];

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub username_key: String,
    #[serde(skip_serializing)]
    pub email: String,
    #[serde(skip_serializing)]
    pub email_key: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[sqlx(try_from = "i64")]
    pub updated_at: u64,
    #[sqlx(try_from = "i64")]
    pub created_at: u64,
}
