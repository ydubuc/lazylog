use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use sqlx::PgPool;

use crate::{app::models::api_error::ApiError, auth::jwt::models::claims::Claims};

use super::{dtos::get_users_filter_dto::GetUsersFilterDto, models::user::User, service};

pub async fn get_users(
    claims: Claims,
    query: Query<GetUsersFilterDto>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<User>>, ApiError> {
    match service::get_users(&query.0, &pool).await {
        Ok(users) => return Ok(Json(users)),
        Err(e) => return Err(e),
    }
}

pub async fn get_user_from_request(
    claims: Claims,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<User>, ApiError> {
    println!("{:?}", claims);

    match service::get_user_by_id(&claims.id, &pool).await {
        Ok(user) => return Ok(Json(user)),
        Err(e) => return Err(e),
    }
}

pub async fn get_user_by_id(
    claims: Claims,
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<User>, ApiError> {
    println!("{:?}", claims);

    match service::get_user_by_id(&id, &pool).await {
        Ok(user) => return Ok(Json(user)),
        Err(e) => return Err(e),
    }
}
