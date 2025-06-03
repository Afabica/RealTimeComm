use jsonwebtoken::{encode, EncodingKey, Header};
use crate::components::models::model_postgres::Token;
use chrono::{Utc, DateTime, Duration};

pub fn generate_jwt(user_id: &str, role: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let hours: u64 = 24;
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24 * 60 * 60))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Token {
        sub: user_id.to_owned(),
        exp: expiration,
        roles: role.to_owned(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub 
