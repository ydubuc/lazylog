use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::{
        self,
        errors::DefaultApiError,
        models::api_error::ApiError,
        util::{argon2::hash_password, sqlx::get_code_from_db_err},
    },
    users::{self, models::user::User},
};

use super::{
    dtos::{login_dto::LoginDto, register_dto::RegisterDto},
    jwt::util::sign_jwt,
    models::access_info::AccessInfo,
};

pub async fn register(dto: &RegisterDto, pool: &PgPool) -> Result<AccessInfo, ApiError> {
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: dto.username.to_string(),
        username_key: dto.username.to_lowercase(),
        email: dto.email.to_string(),
        email_key: dto.email.to_lowercase(),
        password_hash: hash_password(&dto.password),
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

    match sqlx_result {
        Ok(_) => {
            let login_dto = LoginDto {
                username: None,
                email: Some(user.email),
                password: dto.password.to_string(),
            };

            return login(&login_dto, &pool).await;
        }
        Err(e) => match e.as_database_error() {
            Some(db_err) => match get_code_from_db_err(db_err) {
                Some(code) => match code.as_str() {
                    "23505" => {
                        return Err(ApiError {
                            status: StatusCode::CONFLICT,
                            message: "User already exists.".to_string(),
                        })
                    }
                    _ => return Err(DefaultApiError::InternalServerError.value()),
                },
                None => return Err(DefaultApiError::InternalServerError.value()),
            },
            None => return Err(DefaultApiError::InternalServerError.value()),
        },
    }
}

pub async fn login(dto: &LoginDto, pool: &PgPool) -> Result<AccessInfo, ApiError> {
    let user_result = users::service::get_user_by_login_dto(dto, pool).await;

    match user_result {
        Ok(user) => {
            let matches = app::util::argon2::matches(&user.password_hash, &dto.password);
            if !matches {
                return Err(ApiError {
                    status: StatusCode::UNAUTHORIZED,
                    message: "Invalid username or password.".to_string(),
                });
            }

            let access_token = sign_jwt(user.id);
            let access_info = AccessInfo { access_token };

            return Ok(access_info);
        }
        Err(e) => {
            return Err(e);
        }
    }
}
