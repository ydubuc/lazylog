use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

use crate::auth::jwt::models::claims::Claims;

pub fn sign_jwt(id: String) -> String {
    let iat = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let exp = iat + 900;

    let claims = Claims { id, iat, exp };
    let secret = env::var("JWT_SECRET").expect("secret");
    let jwt = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    return jwt;
}

pub fn decode_jwt(jwt: String) -> Result<Claims, Error> {
    let secret = env::var("JWT_SECRET").expect("secret");
    let result = decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(&secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match result {
        Ok(data) => return Ok(data.claims),
        Err(e) => return Err(e),
    }
}
