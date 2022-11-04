use sqlx::PgPool;

use crate::{app::models::api_error::ApiError, auth::jwt::models::claims::Claims};

use super::{dtos::create_media_dto::CreateMediaDto, util::dalle};

pub async fn create_media(
    claims: &Claims,
    dto: &CreateMediaDto,
    pool: &PgPool,
) -> Result<String, ApiError> {
    return dalle::dalle_generate_image(dto).await;
}
