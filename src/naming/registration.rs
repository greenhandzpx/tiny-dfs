use rocket;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct RegisterArg {
    // TODO: &str or string ?
    storage_ip: String,
    // TODO: not sure the size
    client_port: u16,
    command_port: u16,
    files: Vec<String>,
}

#[post("/register", data = "<arg>")]
fn register_storage_server(arg: Json<RegisterArg>) {
    todo!()
}

