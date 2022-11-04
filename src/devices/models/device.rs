use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::users::models::user::User;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: String,
    pub user_id: String,
    #[serde(skip_serializing)]
    pub refresh_token: String,
    #[sqlx(try_from = "i64")]
    pub updated_at: u64,
    #[sqlx(try_from = "i64")]
    pub created_at: u64,
}

impl Device {
    pub fn new(user: &User) -> Self {
        return Self {
            id: Uuid::new_v4().to_string(),
            user_id: user.id.to_string(),
            refresh_token: Uuid::new_v4().to_string(),
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
