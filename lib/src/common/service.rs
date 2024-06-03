use rocket::serde::{Deserialize, Serialize};

use crate::naming::Ip;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct IsValidPathArg {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct IsValidPathResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetStorageArg {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetStorageOkResponse {
    pub server_ip: Ip,
    pub server_port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteArg {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteOkResponse {
    pub success: bool,
}
