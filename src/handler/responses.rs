use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Token {
    pub token: String,
}