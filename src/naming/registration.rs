use std::sync::Arc;

use rocket;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use crate::naming::Ip;

use super::server::{self, StorageServer};

fn register_file(file: String, srv: &Arc<StorageServer>) {}

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

#[derive(Responder)]
enum RegisterResponse {
    OkResp(Json<RegisterOkResponse>),
    ErrResp(Json<RegisterErrResponse>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct RegisterOkResponse {
    files: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct RegisterErrResponse {
    exception_type: String,
    exception_info: String,
}

/// Collect necessary files and retrive all duplicated ones
fn collect_files(files: &Vec<String>) -> Vec<String> {
    todo!()
}

#[post("/register", data = "<arg>")]
pub async fn register_storage_server(arg: Json<RegisterArg>) -> (Status, RegisterResponse) {
    let srv = Arc::new(StorageServer::new(
        Ip(arg.storage_ip.clone()),
        arg.client_port,
        arg.command_port,
    ));
    if server::register_server(srv).await.is_err() {
        return (
            Status::Conflict,
            RegisterResponse::ErrResp(
                RegisterErrResponse {
                    exception_type: "IllegalStateException".to_string(),
                    exception_info: "This storage client already registered.".to_string(),
                }
                .into(),
            ),
        );
    }
    let duplicated_files = collect_files(&arg.files);
    (
        Status::Ok,
        RegisterResponse::OkResp(
            RegisterOkResponse {
                files: duplicated_files,
            }
            .into(),
        ),
    )
}