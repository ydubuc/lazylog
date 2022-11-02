use axum::{extract::Path, Extension, Json};
use sqlx::PgPool;

use crate::{app::models::api_error::ApiError, auth::jwt::models::claims::Claims};

use super::{models::user::User, service};

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
