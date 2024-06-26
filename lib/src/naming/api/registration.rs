use std::sync::Arc;

use rocket;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::common::{
    registration::{RegisterArg, RegisterOkResponse},
    ErrResponse,
};
use crate::naming::{
    dir_tree::collect_files,
    server::{register_server, StorageServer},
    Ip,
};

#[derive(Responder)]
pub enum RegisterResponse {
    OkResp(Json<RegisterOkResponse>),
    ErrResp(Json<ErrResponse>),
}

#[post("/register", data = "<arg>")]
pub async fn register_storage_server(arg: Json<RegisterArg>) -> (Status, RegisterResponse) {
    let srv = Arc::new(StorageServer::new(
        Ip(arg.storage_ip.clone()),
        arg.client_port,
        arg.command_port,
    ));
    if register_server(&srv).await.is_err() {
        return (
            Status::Conflict,
            RegisterResponse::ErrResp(
                ErrResponse {
                    exception_type: "IllegalStateException".to_string(),
                    exception_info: "This storage client already registered.".to_string(),
                }
                .into(),
            ),
        );
    }
    let duplicated_files = collect_files(&arg.files, srv).await.unwrap();
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
