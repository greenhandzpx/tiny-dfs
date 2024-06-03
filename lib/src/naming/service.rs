use rocket::{http::Status, serde::json::Json};

use crate::{
    common::{
        error::ErrResponse,
        service::{
            DeleteArg, DeleteOkResponse, GetStorageArg, GetStorageOkResponse, IsValidPathArg,
            IsValidPathResponse,
        },
    },
    naming::dir_tree,
};

#[post("/is_valid_path", data = "<arg>")]
pub async fn is_valid_path(arg: Json<IsValidPathArg>) -> (Status, Json<IsValidPathResponse>) {
    let path = &arg.path;
    let mut resp = IsValidPathResponse { success: false };

    let (_, target) = dir_tree::lookup(path).await;
    if target.is_some() {
        log::debug!("path {:?} is valid", path);
        resp.success = true;
    } else {
        log::debug!("path {:?} isn't valid", path);
    }
    (Status::Ok, resp.into())
}

#[derive(Responder)]
pub enum GetStorageResponse {
    OkResp(Json<GetStorageOkResponse>),
    ErrResp(Json<ErrResponse>),
}

#[post("/getstorage", data = "<arg>")]
pub async fn get_storage_server(arg: Json<GetStorageArg>) -> (Status, GetStorageResponse) {
    let (_, target) = dir_tree::lookup(&arg.path).await;
    if let Some(target) = target {
        let srv = target.server_ref();
        (
            Status::Ok,
            GetStorageResponse::OkResp(
                GetStorageOkResponse {
                    server_ip: srv.ip.clone(),
                    server_port: srv.client_port,
                }
                .into(),
            ),
        )
    } else {
        // The requested file doesn't exist
        (
            Status::NotFound,
            GetStorageResponse::ErrResp(
                ErrResponse {
                    exception_type: "FileNotFoundException".to_string(),
                    exception_info: format!("{} cannot be found", &arg.path),
                }
                .into(),
            ),
        )
    }
}

#[derive(Responder)]
pub enum DeleteResponse {
    OkResp(Json<DeleteOkResponse>),
    ErrResp(Json<ErrResponse>),
}

#[post("/delete", data = "<arg>")]
pub async fn delete_file(arg: Json<DeleteArg>) -> (Status, DeleteResponse) {
    if dir_tree::delete_file(&arg.path).await.is_err() {
        (
            Status::Ok,
            DeleteResponse::OkResp(DeleteOkResponse { success: true }.into()),
        )
    } else {
        // Delete failed
        return (
            Status::NotFound,
            DeleteResponse::ErrResp(
                ErrResponse {
                    exception_type: "FileNotFoundException".to_string(),
                    exception_info: "the object or parent directory does not exist.".to_string(),
                }
                .into(),
            ),
        );
    }
}
