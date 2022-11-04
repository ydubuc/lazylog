use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};

use crate::{app::models::api_error::ApiError, auth::jwt::util::decode_jwt};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: String,
    pub iat: u64,
    pub exp: u64,
}

impl Claims {
    pub fn from_header(authorization: Authorization<Bearer>) -> Result<Self, ApiError> {
        match decode_jwt(authorization.0.token().to_string()) {
            Ok(claims) => return Ok(claims),
            Err(e) => match e {
                ErrorKind::ExpiredSignature => {
                    return Err(ApiError {
                        code: StatusCode::UNAUTHORIZED,
                        message: "Token expired".to_string(),
                    });
                }
                _ => {
                    return Err(ApiError {
                        code: StatusCode::UNAUTHORIZED,
                        message: "Invalid token.".to_string(),
                    });
                }
            },
        }
    }
}

// #[async_trait]
// impl<S, B> FromRequest<S, B> for Claims
// where
//     B: Send + 'static,
//     S: Send + Sync,
// {
//     type Rejection = ApiError;

//     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//         let TypedHeader(Authorization(bearer_token)) =
//             TypedHeader::<Authorization<Bearer>>::from_request(req, state)
//                 .await
//                 .map_err(|_| ApiError {
//                     code: StatusCode::UNAUTHORIZED,
//                     message: "Missing token.".to_string(),
//                 })?;

//         match decode_jwt(bearer_token.token().to_string()) {
//             Ok(claims) => return Ok(claims),
//             Err(e) => match e {
//                 ErrorKind::ExpiredSignature => {
//                     return Err(ApiError {
//                         code: StatusCode::UNAUTHORIZED,
//                         message: "Token expired".to_string(),
//                     });
//                 }
//                 _ => {
//                     return Err(ApiError {
//                         code: StatusCode::UNAUTHORIZED,
//                         message: "Invalid token.".to_string(),
//                     });
//                 }
//             },
//         }
//     }
// }

// #[async_trait]
// impl<B> FromRequest<B> for Claims
// where
//     B: Send,
// {
//     type Rejection = ApiError;

//     async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
//         let TypedHeader(Authorization(bearer_token)) =
//             TypedHeader::<Authorization<Bearer>>::from_request(req)
//                 .await
//                 .map_err(|_| ApiError {
//                     code: StatusCode::UNAUTHORIZED,
//                     message: "Missing token.".to_string(),
//                 })?;

//         match decode_jwt(bearer_token.token().to_string()) {
//             Ok(claims) => return Ok(claims),
//             Err(e) => match e {
//                 ErrorKind::ExpiredSignature => {
//                     return Err(ApiError {
//                         code: StatusCode::UNAUTHORIZED,
//                         message: "Token expired".to_string(),
//                     });
//                 }
//                 _ => {
//                     return Err(ApiError {
//                         code: StatusCode::UNAUTHORIZED,
//                         message: "Invalid token.".to_string(),
//                     });
//                 }
//             },
//         }
//     }
// }
