use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::naming::Ip;

use super::{ErrResponse, OkResponse};

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

#[derive(Responder)]
pub enum DeleteResponse {
    OkResp(Json<OkResponse>),
    ErrResp(Json<ErrResponse>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateDirectoryArg {
    pub path: String,
}

#[derive(Responder)]
pub enum CreateDirectoryResponse {
    OkResp(Json<OkResponse>),
    ErrResp(Json<ErrResponse>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateFileArg {
    pub path: String,
}

#[derive(Responder)]
pub enum CreateFileResponse {
    OkResp(Json<OkResponse>),
    ErrResp(Json<ErrResponse>),
}
