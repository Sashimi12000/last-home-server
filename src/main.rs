mod error;
mod handler;

use error::AppError;
use confy;
use handler::{ApiResponse, Responses, Token};
use serde_derive::{Deserialize, Serialize};
use axum::{extract, response::{self, Response}, routing, Router};
use tokio;
use jsonwebtoken::{encode, EncodingKey};
use chrono::{DateTime, Duration, FixedOffset, TimeZone, Utc};
use clap::Parser;



#[derive(Serialize, Deserialize, Debug)]
struct  AppConfig {
    version: i32,
    port: String,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 1,
            port: "3000".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct UserForm {
    id: String,
    pass: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    iat: i64,
    exp: i64,
    uuid: String,

}

#[derive(Parser)]
pub struct Env {
    #[arg(env, hide_env_values = true)]
    pub secret: String,
}

#[tokio::main]
async fn main() -> Result<(), AppError>{
    let cfg: AppConfig = confy::load("config", None)?;
    let addr = format!("127.0.0.1:{}", cfg.port);
    let app = Router::new()
        .route("/token", routing::post(login));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}


async fn login(extract::Json(user): extract::Json<UserForm>) -> response::Json<ApiResponse> {
    let mut response = ApiResponse::default();
    let env = Env::parse();
    let Ok(token) = generate_token(user, &env.secret).await else {
        response::Json(ApiResponse::new(Responses::Error, "801 Token Generate Failed"));
    };
    let res = Token {token: token};
    response::Json(ApiResponse::new(Responses::Token(res), "200 OK"))

}

async fn generate_token(user: UserForm, secret: &str) -> Result<String, AppError> {
    let dt = Utc::now();
    let mut header = jsonwebtoken::Header::default();
    header.typ = Some(String::from("JWT"));
    header.alg = jsonwebtoken::Algorithm::HS256;
    let claim = Claims {
        iat: dt.timestamp(),
        exp: (dt + Duration::hours(24)).timestamp(),
        uuid: user.id.to_string(),
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