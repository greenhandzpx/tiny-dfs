use rocket::serde::{Deserialize, Serialize};

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
