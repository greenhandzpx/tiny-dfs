use std::sync::Arc;

use rocket::{http::Status, serde::json::Json};

use crate::{
    common::{
        error::ErrResponse,
        service::{
            CreateDirectoryArg, CreateDirectoryResponse, DeleteArg, DeleteOkResponse,
            DeleteResponse, GetStorageArg, GetStorageOkResponse, IsValidPathArg,
            IsValidPathResponse,
        },
    },
    naming::dir_tree,
};

use super::server::StorageServer;

#[post("/is_valid_path", data = "<arg>")]
pub async fn is_valid_path(arg: Json<IsValidPathArg>) -> (Status, Json<IsValidPathResponse>) {
    let path = &arg.path;
    let mut resp = IsValidPathResponse { success: false };

    let res = dir_tree::lookup(path).await;
    if let Some((_, target)) = res.ok() {
        if target.is_some() {
            log::debug!("path {:?} is valid", path);
            resp.success = true;
        } else {
            log::debug!("path {:?} isn't valid", path);
        }
    }
    (Status::Ok, resp.into())
}

#[derive(Responder)]
pub enum GetStorageResponse {
    OkResp(Json<GetStorageOkResponse>),
    ErrResp(Json<ErrResponse>),
}

/// TODO: achieve load-balancing
fn select_one_server(srvs: &mut Vec<Arc<StorageServer>>) -> Arc<StorageServer> {
    srvs[0].clone()
}

#[post("/getstorage", data = "<arg>")]
pub async fn get_storage_server(arg: Json<GetStorageArg>) -> (Status, GetStorageResponse) {
    let res = dir_tree::lookup(&arg.path).await;
    if res.is_err() {
        return (
            Status::NotFound,
            GetStorageResponse::ErrResp(
                ErrResponse {
                    exception_type: "FileNotFoundException".to_string(),
                    exception_info: format!("{} is invalid", &arg.path),
                }
                .into(),
            ),
        );
    }
    let (_, target) = res.ok().unwrap();
    if let Some(target) = target {
        let srv = target.for_all_servers(select_one_server);
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

#[post("/delete", data = "<arg>")]
pub async fn delete_file(arg: Json<DeleteArg>) -> (Status, DeleteResponse) {
    if let Some(target) = dir_tree::delete_file(&arg.path).await.ok() {
        // TODO: inform the storage server periodically
        // Broadcast all storage servers to delete this file
        let mut tasks = Vec::new();
        target.for_all_servers(|servers| {
            // TODO: use a more efficient way to inform all servers in parallel
            for srv in servers {
                let arg = DeleteArg {
                    path: arg.path.clone(),
                };
                let client = reqwest::Client::new();
                let addr = format!("http://{}:{}/storage_delete", srv.ip.0, srv.command_port);
                let task = rocket::tokio::spawn(async move {
                    let resp = client.post(addr).json(&arg).send().await.unwrap();
                    assert!(resp.status().is_success());
                });
                tasks.push(task);
            }
        });
        for task in tasks {
            task.await.unwrap();
        }
        (
            Status::Ok,
            DeleteResponse::OkResp(DeleteOkResponse { success: true }.into()),
        )
    } else {
        // Delete failed
        (
            Status::NotFound,
            DeleteResponse::ErrResp(
                ErrResponse {
                    exception_type: "FileNotFoundException".to_string(),
                    exception_info: "the object or parent directory does not exist.".to_string(),
                }
                .into(),
            ),
        )
    }
}

// #[post("/create_directory", data = "<arg>")]
// pub async fn create_directory(arg: Json<CreateDirectoryArg>) -> (Status, CreateDirectoryResponse) {
//     dir_tree::create_file(&arg.path, true, , )
//     todo!()
// }
