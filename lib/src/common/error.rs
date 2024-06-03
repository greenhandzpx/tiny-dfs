use rocket::serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum TinyDfsError {
    StorageServerExists,
    FileExists,
    FileNotFound,
    DirNotFound,
    DirReadErr,
    RegisterFailed,
    // TODO
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrResponse {
    pub exception_type: String,
    pub exception_info: String,
}
