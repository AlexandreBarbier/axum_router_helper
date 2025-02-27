use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub iat: usize,
    pub exp: usize,
    pub user_id: String,
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

pub fn encode_jwt(user_id: String) -> Result<String, i32> {
    let secret: String =
        std::env::var("JWT_SECRET").unwrap_or("randomStringTypicallyFromEnv".to_string());
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, user_id };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| 401)
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, i32> {
    let secret = std::env::var("JWT_SECRET").unwrap_or("randomStringTypicallyFromEnv".to_string());
    decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| 401)
}
