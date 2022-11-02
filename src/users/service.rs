use axum::http::StatusCode;
use sqlx::PgPool;

use crate::{app::models::api_error::ApiError, auth::dtos::login_dto::LoginDto};

use super::{errors::UsersApiError, models::user::User};

pub async fn get_user_by_id(id: &str, pool: &PgPool) -> Result<User, ApiError> {
    let sqlx_result = sqlx::query_as::<_, User>(
        "
        SELECT * FROM users WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(pool)
    .await;

    match sqlx_result {
        Ok(user) => match user {
            Some(user) => return Ok(user),
            None => return Err(UsersApiError::UserNotFound.value()),
        },
        Err(_) => return Err(UsersApiError::UserNotFound.value()),
    }
}

pub async fn get_user_by_login_dto(login_dto: &LoginDto, pool: &PgPool) -> Result<User, ApiError> {
    if let Some(username) = &login_dto.username {
        return get_user_by_username(username, pool).await;
    }
    if let Some(email) = &login_dto.email {
        return get_user_by_email(email, pool).await;
    }

    return Err(ApiError {
        status: StatusCode::BAD_REQUEST,
        message: "Missing credentials.".to_string(),
    });
}

pub async fn get_user_by_username(username: &str, pool: &PgPool) -> Result<User, ApiError> {
    let sqlx_result = sqlx::query_as::<_, User>(
        "
        SELECT * FROM users WHERE username_key = $1
        ",
    )
    .bind(username.to_lowercase())
    .fetch_optional(pool)
    .await;

    match sqlx_result {
        Ok(user) => match user {
            Some(user) => return Ok(user),
            None => return Err(UsersApiError::UserNotFound.value()),
        },
        Err(_) => return Err(UsersApiError::UserNotFound.value()),
    }
}

pub async fn get_user_by_email(email: &str, pool: &PgPool) -> Result<User, ApiError> {
    let sqlx_result = sqlx::query_as::<_, User>(
        "
        SELECT * FROM users WHERE email_key = $1
        ",
    )
    .bind(email.to_lowercase())
    .fetch_optional(pool)
    .await;

    match sqlx_result {
        Ok(user) => match user {
            Some(user) => return Ok(user),
            None => return Err(UsersApiError::UserNotFound.value()),
        },
        Err(_) => return Err(UsersApiError::UserNotFound.value()),
    }
}
