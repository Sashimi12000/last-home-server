mod responses;

use axum::http::Error;
use chrono::Utc;
use serde_derive::Serialize;

pub use self::responses::{Token};

#[derive(Serialize)]
pub enum Responses {
    Token(Token),
    Error,
}




#[derive(Serialize)]
pub struct ApiResponse {
    status: String,
    timestamp: i64,
    response: Responses,
}
impl ApiResponse {
    pub fn new(res: Responses, status: &str) -> ApiResponse {
        Self {
            status: status.to_string(),
            timestamp: (Utc::now()).timestamp(),
            response: res,
        }
    }
    pub fn default() -> ApiResponse {
        Self::new(Responses::Error, "500 Internal Server Error")
    }
}