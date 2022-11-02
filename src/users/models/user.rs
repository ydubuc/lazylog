use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub username_key: String,
    pub email: String,
    pub email_key: String,
    pub password_hash: String,
    #[sqlx(try_from = "i64")]
    pub updated_at: u64,
    #[sqlx(try_from = "i64")]
    pub created_at: u64,
}
