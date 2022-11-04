use serde_json::json;

extern crate reqwest;

use crate::{app::models::api_error::ApiError, media::dtos::create_media_dto::CreateMediaDto};
use reqwest::{header, StatusCode};

pub async fn dalle_generate_image(dto: &CreateMediaDto) -> Result<String, ApiError> {
    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        ["Bearer ", &openai_api_key].concat().parse().unwrap(),
    );

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/images/generations")
        .headers(headers)
        .body(
            json!({
                "prompt": dto.prompt,
                "n": dto.number.or(Some(1)),
                "size": "512x512"
            })
            .to_string(),
        )
        .send()
        .await;

    match res {
        Ok(res) => match res.text().await {
            Ok(text) => Ok(text),
            Err(e) => Err(ApiError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: e.to_string(),
            }),
        },
        Err(e) => Err(ApiError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }),
    }
}
