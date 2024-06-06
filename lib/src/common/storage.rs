use base64::{
    alphabet,
    engine::{general_purpose::PAD, GeneralPurpose},
    DecodeError, Engine,
};
use rocket::serde::{json::Json, Deserialize, Serialize};

use super::{ErrResponse, OkResponse, PathArg};

pub type SizeArg = PathArg;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SizeOkResponse {
    pub size: u64,
}

#[derive(Responder)]
pub enum SizeResponse {
    OkResp(Json<SizeOkResponse>),
    ErrResp(Json<ErrResponse>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReadArg {
    pub path: String,
    pub offset: u64,
    pub length: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReadOkResponse {
    pub data: String,
}

#[derive(Responder)]
pub enum ReadResponse {
    OkResp(Json<ReadOkResponse>),
    ErrResp(Json<ErrResponse>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct WriteArg {
    pub path: String,
    pub offset: u64,
    pub data: String,
}

#[derive(Responder)]
pub enum WriteResponse {
    OkResp(Json<OkResponse>),
    ErrResp(Json<ErrResponse>),
}

const BASE64_ENGINE: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, PAD);

pub fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    BASE64_ENGINE.encode(input)
}

pub fn base64_decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    BASE64_ENGINE.decode(input)
}
