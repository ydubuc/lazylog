use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessInfo {
    pub access_token: String,
    pub refresh_token: Option<String>,
}
