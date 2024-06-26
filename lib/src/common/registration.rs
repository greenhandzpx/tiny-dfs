use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterArg {
    // TODO: &str or string ?
    pub storage_ip: String,
    pub client_port: u16,
    pub command_port: u16,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterOkResponse {
    pub files: Vec<String>,
}
