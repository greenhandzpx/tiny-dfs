use rocket::serde::{Deserialize, Serialize};

pub mod error;
pub mod registration;
pub mod service;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrResponse {
    pub exception_type: String,
    pub exception_info: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct OkResponse {
    pub success: bool,
}
