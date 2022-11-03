use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::{
        errors::DefaultApiError,
        models::{api_error::ApiError, sql_state_codes::SqlStateCodes},
        util::sqlx::get_code_from_db_err,
    },
    users::models::user::User,
};

use super::{dtos::refresh_device_dto::RefreshDeviceDto, models::device::Device};

pub async fn create_device(user: &User, pool: &PgPool) -> Result<Device, ApiError> {
    let device = Device {
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

    let sqlx_result = sqlx::query_as::<_, Device>(
        "
        INSERT INTO devices (
            id, user_id, refresh_token, updated_at, created_at
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        ",
    )
    .bind(&device.id)
    .bind(&device.user_id)
    .bind(&device.refresh_token)
    .bind(device.updated_at.to_owned() as i64)
    .bind(device.created_at.to_owned() as i64)
    .fetch_one(pool)
    .await;

    if let Some(error) = sqlx_result.as_ref().err() {
        println!("{}", error);
    }

    match sqlx_result {
        Ok(_) => Ok(device),
        Err(e) => {
            let Some(db_err) = e.as_database_error() else {
                return Err(DefaultApiError::InternalServerError.value());
            };

            let Some(code) = get_code_from_db_err(db_err) else {
                return Err(DefaultApiError::InternalServerError.value());
            };

            match code.as_str() {
                SqlStateCodes::UniqueViolation => {
                    return Err(ApiError {
                        status: StatusCode::CONFLICT,
                        message: "Device already exists.".to_string(),
                    })
                }
                _ => return Err(DefaultApiError::InternalServerError.value()),
            }
        }
    }
}

pub async fn refresh_device(dto: &RefreshDeviceDto, pool: &PgPool) -> Result<(), ApiError> {
    let sqlx_result = sqlx::query(
        "
        UPDATE devices SET updated_at = $1
        WHERE id = $2 AND user_id = $3 AND refresh_token = $4
        ",
    )
    .bind(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    )
    .bind(&dto.device_id)
    .bind(&dto.user_id)
    .bind(&dto.refresh_token)
    .execute(pool)
    .await;

    if let Some(error) = sqlx_result.as_ref().err() {
        println!("{}", error);
    }

    match sqlx_result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                return Ok(());
            } else {
                return Err(ApiError {
                    status: StatusCode::NOT_FOUND,
                    message: "Failed to refresh.".to_string(),
                });
            }
        }
        Err(_) => return Err(DefaultApiError::InternalServerError.value()),
    }
}
