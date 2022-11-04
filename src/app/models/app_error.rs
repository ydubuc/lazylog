use axum::http::StatusCode;

use super::api_error::ApiError;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl AppError {
    fn as_api_error(self) -> ApiError {
        return ApiError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: self.message,
        };
    }
}
