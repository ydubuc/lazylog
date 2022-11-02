use axum::http::StatusCode;

use crate::app::models::api_error::ApiError;

#[derive(Debug)]
pub enum UsersApiError {
    UserNotFound,
}

impl UsersApiError {
    pub fn value(&self) -> ApiError {
        match *self {
            Self::UserNotFound => ApiError {
                status: StatusCode::NOT_FOUND,
                message: "User not found.".to_string(),
            },
        }
    }
}
