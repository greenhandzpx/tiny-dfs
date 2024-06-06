use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::naming::Ip;

use super::{ErrResponse, OkResponse, PathArg};

pub type IsValidPathArg = PathArg;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct IsValidPathResponse {
    pub success: bool,
}

pub type GetStorageArg = PathArg;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetStorageOkResponse {
    pub server_ip: Ip,
    pub server_port: u16,
}

pub type DeleteArg = PathArg;

#[derive(Responder)]
pub enum DeleteResponse {
    OkResp(Json<OkResponse>),
    ErrResp(Json<ErrResponse>),
}

pub type CreateDirectoryArg = PathArg;

#[derive(Responder)]
pub enum CreateDirectoryResponse {
    OkResp(Json<OkResponse>),
    ErrResp(Json<ErrResponse>),
}

pub type CreateFileArg = PathArg;

#[derive(Responder)]
pub enum CreateFileResponse {
    OkResp(Json<OkResponse>),
    ErrResp(Json<ErrResponse>),
}

pub type ListArg = PathArg;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ListOkResponse {
    pub files: Vec<String>,
}

#[derive(Responder)]
pub enum ListResponse {
    OkResp(Json<ListOkResponse>),
    ErrResp(Json<ErrResponse>),
}

pub type IsDirectoryArg = PathArg;

#[derive(Responder)]
pub enum IsDirectoryResponse {
    OkResp(Json<OkResponse>),
    ErrResp(Json<ErrResponse>),
}
