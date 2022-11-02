use axum::http::StatusCode;
use sqlx::PgPool;

use crate::{
    app::{self, models::api_error::ApiError},
    devices::{self, dtos::refresh_device_dto::RefreshDeviceDto},
    users,
};

use super::{
    dtos::{login_dto::LoginDto, register_dto::RegisterDto},
    jwt::util::sign_jwt,
    models::access_info::AccessInfo,
};

pub async fn register(dto: &RegisterDto, pool: &PgPool) -> Result<AccessInfo, ApiError> {
    match users::service::create_user(dto, pool).await {
        Ok(_) => {
            let login_dto = LoginDto {
                username: None,
                email: Some(dto.email.to_string()),
                password: dto.password.to_string(),
            };

            return login(&login_dto, &pool).await;
        }
        Err(e) => Err(e),
    }
}

pub async fn login(dto: &LoginDto, pool: &PgPool) -> Result<AccessInfo, ApiError> {
    match users::service::get_user_by_login_dto(dto, pool).await {
        Ok(user) => {
            let matches = app::util::argon2::matches(&user.password_hash, &dto.password);
            if !matches {
                return Err(ApiError {
                    status: StatusCode::UNAUTHORIZED,
                    message: "Invalid password.".to_string(),
                });
            }

            match devices::service::create_device(&user, pool).await {
                Ok(device) => {
                    let access_info = AccessInfo {
                        access_token: sign_jwt(&user.id),
                        refresh_token: Some(device.refresh_token),
                    };

                    return Ok(access_info);
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub async fn refresh(dto: &RefreshDeviceDto, pool: &PgPool) -> Result<AccessInfo, ApiError> {
    let result = devices::service::refresh_device(dto, pool).await;

    match result {
        Ok(_) => {
            return Ok(AccessInfo {
                access_token: sign_jwt(&dto.user_id),
                refresh_token: None,
            })
        }
        Err(_) => {
            return Err(ApiError {
                status: StatusCode::NOT_FOUND,
                message: "Failed to refresh".to_string(),
            })
        }
    }
}
