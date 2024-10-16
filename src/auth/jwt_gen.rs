
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::auth::claims::Claims;

fn create_jwt(secret: &[u8], user_id: &str, company: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 60 * 60;

    let claims = Claims {
        sub: user_id.to_owned(),
        company: company.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).unwrap()
}

fn validate_jwt(token: &str, secret: &[u8]) -> Result<Claims, Error> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &validation,
    )?;

    Ok(token_data.claims)
}