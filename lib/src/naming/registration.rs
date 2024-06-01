use std::sync::Arc;

use rocket;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::common::registration::{RegisterArg, RegisterErrResponse, RegisterOkResponse};
use crate::naming::Ip;

use super::dir_tree::collect_files;
use super::server::{self, StorageServer};

#[derive(Responder)]
pub enum RegisterResponse {
    OkResp(Json<RegisterOkResponse>),
    ErrResp(Json<RegisterErrResponse>),
}

#[post("/register", data = "<arg>")]
pub async fn register_storage_server(arg: Json<RegisterArg>) -> (Status, RegisterResponse) {
    let srv = Arc::new(StorageServer::new(
        Ip(arg.storage_ip.clone()),
        arg.client_port,
        arg.command_port,
    ));
    if server::register_server(&srv).await.is_err() {
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
    let duplicated_files = collect_files(&arg.files, &srv).await.unwrap();
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
