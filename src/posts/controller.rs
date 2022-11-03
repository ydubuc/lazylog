use axum::{Extension, Json};
use sqlx::PgPool;

use crate::{app::models::api_error::ApiError, auth::jwt::models::claims::Claims};

use super::{dtos::create_post_dto::CreatePostDto, models::post::Post, service};

pub async fn create_post(
    claims: Claims,
    Json(dto): Json<CreatePostDto>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Post>, ApiError> {
    match service::create_post(&claims, &dto, &pool).await {
        Ok(post) => return Ok(Json(post)),
        Err(e) => return Err(e),
    }
}
