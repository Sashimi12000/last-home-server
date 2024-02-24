mod error;
mod handler;
mod token;

use error::AppError;
use handler::{ApiResponse, Responses, Token};
use token::token as generate_token;

use confy;
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

#[derive(Parser)]
pub struct Env {
    #[arg(env, hide_env_values = true)]
    pub secret: String,
}

#[derive(Deserialize)]
struct UserForm {
    id: String,
    pass: String,
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
    if let Ok(token) = generate_token(user, &env.secret).await {
        response::Json(ApiResponse::new(Responses::Error, "801 Token Generate Failed"))
    } else {
    let response_token = Token {token: token};
    response::Json(ApiResponse::new(Responses::Token(response_token), "200 OK"))
    }
}

