use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use validator::Validate;

use crate::{
    app::models::{api_error::ApiError, json_from_request::JsonFromRequest},
    auth::jwt::models::claims::Claims,
    AppState,
};

use super::{dtos::create_media_dto::CreateMediaDto, service};

pub async fn create_media(
    State(state): State<AppState>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    JsonFromRequest(dto): JsonFromRequest<CreateMediaDto>,
) -> Result<String, ApiError> {
    match Claims::from_header(authorization) {
        Ok(claims) => match dto.validate() {
            Ok(_) => service::create_media(&claims, &dto, &state.pool).await,
            Err(e) => Err(ApiError {
                code: StatusCode::BAD_REQUEST,
                message: e.to_string(),
            }),
        },
        Err(e) => Err(e),
    }
}
