use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RefreshDeviceDto {
    pub device_id: String,
    pub user_id: String,
    pub refresh_token: String,
}
