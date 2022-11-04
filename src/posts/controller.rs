use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    Json, TypedHeader,
};

use crate::{
    app::models::{api_error::ApiError, json_from_request::JsonFromRequest},
    auth::jwt::models::claims::Claims,
    AppState,
};

use super::{dtos::create_post_dto::CreatePostDto, models::post::Post, service};

pub async fn create_post(
    State(state): State<AppState>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    JsonFromRequest(dto): JsonFromRequest<CreatePostDto>,
) -> Result<Json<Post>, ApiError> {
    match Claims::from_header(authorization) {
        Ok(claims) => match service::create_post(&claims, &dto, &state.pool).await {
            Ok(post) => return Ok(Json(post)),
            Err(e) => return Err(e),
        },
        Err(e) => Err(e),
    }
}
