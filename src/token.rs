use serde_derive::{Serialize, Deserialize};
use chrono::{DateTime, Duration, FixedOffset, TimeZone, Utc};
use jsonwebtoken::{encode, EncodingKey};

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
struct Claims {
    iat: i64,
    exp: i64,
    uuid: String,

}


pub async fn token(user: String, secret: &str) -> Result<String, AppError> {
    let dt = Utc::now();
    let mut header = jsonwebtoken::Header::default();
    header.typ = Some(String::from("JWT"));
    header.alg = jsonwebtoken::Algorithm::HS256;
    let claim = Claims {
        iat: dt.timestamp(),
        exp: (dt + Duration::hours(24)).timestamp(),
        uuid: user.to_string(),
    };
    match encode(&header, &claim, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(token) => {
            return Ok(token)
        },
        Err(e) => {
            return Err(AppError::JsonWebtoken(e))
        }
    }
}