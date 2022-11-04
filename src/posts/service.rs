use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::{
        errors::DefaultApiError,
        models::api_error::ApiError,
        util::sqlx::{get_code_from_db_err, SqlStateCodes},
    },
    auth::jwt::models::claims::Claims,
};

use super::{dtos::create_post_dto::CreatePostDto, models::post::Post};

pub async fn create_post(
    claims: &Claims,
    dto: &CreatePostDto,
    pool: &PgPool,
) -> Result<Post, ApiError> {
    let post = Post {
        id: Uuid::new_v4().to_string(),
        user_id: claims.id.to_string(),
        title: dto.title.to_string(),
        content: dto.content.to_owned(),
        updated_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    let sqlx_result = sqlx::query(
        "
        INSERT INTO posts (
            id, user_id, title, content, updated_at, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ",
    )
    .bind(&post.id)
    .bind(&post.user_id)
    .bind(&post.title)
    .bind(&post.content)
    .bind(post.updated_at.to_owned() as i64)
    .bind(post.created_at.to_owned() as i64)
    .execute(pool)
    .await;

    if let Some(error) = sqlx_result.as_ref().err() {
        println!("{}", error);
    }

    match sqlx_result {
        Ok(_) => Ok(post),
        Err(e) => {
            let Some(db_err) = e.as_database_error() else {
                return Err(DefaultApiError::InternalServerError.value());
            };

            let Some(code) = get_code_from_db_err(db_err) else {
                return Err(DefaultApiError::InternalServerError.value());
            };

            match code.as_str() {
                SqlStateCodes::UNIQUE_VIOLATION => {
                    return Err(ApiError {
                        code: StatusCode::CONFLICT,
                        message: "Post already exists.".to_string(),
                    })
                }
                _ => return Err(DefaultApiError::InternalServerError.value()),
            }
        }
    }
}
