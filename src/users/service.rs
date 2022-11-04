use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::{
        errors::DefaultApiError,
        models::api_error::ApiError,
        util::{
            hasher,
            sqlx::{get_code_from_db_err, SqlStateCodes},
        },
    },
    auth::dtos::{login_dto::LoginDto, register_dto::RegisterDto},
};

use super::{
    dtos::get_users_filter_dto::GetUsersFilterDto, errors::UsersApiError, models::user::User,
};

pub async fn create_user(dto: &RegisterDto, pool: &PgPool) -> Result<User, ApiError> {
    let Ok(hash) = hasher::hash(dto.password.to_string()).await else {
        return Err(ApiError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to hash password.".to_string()
        })
    };

    let user = User {
        id: Uuid::new_v4().to_string(),
        username: dto.username.to_string(),
        username_key: dto.username.to_lowercase(),
        email: dto.email.to_string(),
        email_key: dto.email.to_lowercase(),
        password_hash: hash,
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
        INSERT INTO users (
            id, username, username_key, email, email_key, password_hash, updated_at, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ",
    )
    .bind(&user.id)
    .bind(&user.username)
    .bind(&user.username_key)
    .bind(&user.email)
    .bind(&user.email_key)
    .bind(&user.password_hash)
    .bind(user.updated_at.to_owned() as i64)
    .bind(user.created_at.to_owned() as i64)
    .execute(pool)
    .await;

    if let Some(error) = sqlx_result.as_ref().err() {
        println!("{}", error);
    }

    match sqlx_result {
        Ok(_) => Ok(user),
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
                        message: "User already exists.".to_string(),
                    })
                }
                _ => return Err(DefaultApiError::InternalServerError.value()),
            }
        }
    }
}

pub async fn get_users(dto: &GetUsersFilterDto, pool: &PgPool) -> Result<Vec<User>, ApiError> {
    let Ok(sql) = dto.to_sql() else {
        return Err(ApiError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to parse query".to_string()
        });
    };

    let mut sqlx = sqlx::query_as::<_, User>(&sql);

    if let Some(id) = &dto.id {
        sqlx = sqlx.bind(id);
    }
    if let Some(username) = &dto.username {
        sqlx = sqlx.bind(["%", &username.to_lowercase(), "%"].concat());
    }

    let sqlx_result = sqlx.fetch_all(pool).await;

    if let Some(error) = sqlx_result.as_ref().err() {
        println!("{}", error);
    }

    match sqlx_result {
        Ok(users) => return Ok(users),
        Err(_) => return Err(DefaultApiError::InternalServerError.value()),
    }
}

pub async fn get_user_by_id(id: &str, pool: &PgPool) -> Result<User, ApiError> {
    let sqlx_result = sqlx::query_as::<_, User>(
        "
        SELECT * FROM users WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(pool)
    .await;

    if let Some(error) = sqlx_result.as_ref().err() {
        println!("{}", error);
    }

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
        code: StatusCode::BAD_REQUEST,
        message: "Missing credentials.".to_string(),
    });
}

pub async fn get_user_by_username(username: &str, pool: &PgPool) -> Result<User, ApiError> {
    let sqlx_result = sqlx::query_as::<_, User>(
        "
        SELECT * FROM users
        WHERE username_key = $1
        ",
    )
    .bind(username.to_lowercase())
    .fetch_optional(pool)
    .await;

    if let Some(error) = sqlx_result.as_ref().err() {
        println!("{}", error);
    }

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
        SELECT * FROM users
        WHERE email_key = $1
        ",
    )
    .bind(email.to_lowercase())
    .fetch_optional(pool)
    .await;

    if let Some(error) = sqlx_result.as_ref().err() {
        println!("{}", error);
    }

    match sqlx_result {
        Ok(user) => match user {
            Some(user) => return Ok(user),
            None => return Err(UsersApiError::UserNotFound.value()),
        },
        Err(_) => return Err(UsersApiError::UserNotFound.value()),
    }
}
