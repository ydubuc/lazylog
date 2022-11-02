use axum::{Extension, Json};
use sqlx::PgPool;

use crate::{
    app::models::api_error::ApiError, devices::dtos::refresh_device_dto::RefreshDeviceDto,
};

use super::{
    dtos::{login_dto::LoginDto, register_dto::RegisterDto},
    models::access_info::AccessInfo,
    service,
};

pub async fn register(
    Json(dto): Json<RegisterDto>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<AccessInfo>, ApiError> {
    match service::register(&dto, &pool).await {
        Ok(user) => return Ok(Json(user)),
        Err(e) => return Err(e),
    };
}

pub async fn login(
    Json(dto): Json<LoginDto>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<AccessInfo>, ApiError> {
    match service::login(&dto, &pool).await {
        Ok(user) => return Ok(Json(user)),
        Err(e) => return Err(e),
    }
}

pub async fn refresh(
    Json(dto): Json<RefreshDeviceDto>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<AccessInfo>, ApiError> {
    match service::refresh(&dto, &pool).await {
        Ok(access_info) => return Ok(Json(access_info)),
        Err(e) => return Err(e),
    }
}
