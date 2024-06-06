use std::sync::Arc;

use rand::Rng;
use rocket::{http::Status, serde::json::Json};

use crate::{
    common::{
        error::TinyDfsError,
        service::{
            CreateDirectoryArg, CreateDirectoryResponse, CreateFileArg, CreateFileResponse,
            DeleteArg, DeleteResponse, GetStorageArg, GetStorageOkResponse, IsDirectoryArg,
            IsDirectoryResponse, IsValidPathArg, IsValidPathResponse, ListArg, ListOkResponse,
            ListResponse,
        },
        ErrResponse, OkResponse,
    },
    naming::{
        dir_tree,
        server::{select_random_server, StorageServer},
    },
};

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
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..srvs.len());
    srvs[idx].clone()
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
            DeleteResponse::OkResp(OkResponse { success: true }.into()),
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

#[post("/create_directory", data = "<arg>")]
pub async fn create_directory(arg: Json<CreateDirectoryArg>) -> (Status, CreateDirectoryResponse) {
    match dir_tree::create_file(&arg.path, true, None, false).await {
        Err(err) => {
            let (status, exception_type, exception_info) = err.exception();
            return (
                status,
                CreateDirectoryResponse::ErrResp(
                    ErrResponse {
                        exception_type: exception_type.to_string(),
                        exception_info: exception_info.to_string(),
                    }
                    .into(),
                ),
            );
        }
        Ok(_) => {
            return (
                Status::Ok,
                CreateDirectoryResponse::OkResp(OkResponse { success: true }.into()),
            );
        }
    }
}

#[post("/create_file", data = "<arg>")]
pub async fn create_file(arg: Json<CreateFileArg>) -> (Status, CreateFileResponse) {
    let srv = select_random_server().await;
    assert!(srv.is_some());
    match dir_tree::create_file(&arg.path, false, srv, false).await {
        Err(err) => {
            let (status, exception_type, exception_info) = err.exception();
            return (
                status,
                CreateFileResponse::ErrResp(
                    ErrResponse {
                        exception_type: exception_type.to_string(),
                        exception_info: exception_info.to_string(),
                    }
                    .into(),
                ),
            );
        }
        Ok(target) => {
            // Broadcast all storage servers to create this file
            let mut tasks = Vec::new();
            target.for_all_servers(|servers| {
                // TODO: use a more efficient way to inform all servers in parallel
                for srv in servers {
                    let arg = CreateFileArg {
                        path: arg.path.clone(),
                    };
                    let client = reqwest::Client::new();
                    let addr = format!("http://{}:{}/storage_create", srv.ip.0, srv.command_port);
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
            return (
                Status::Ok,
                CreateFileResponse::OkResp(OkResponse { success: true }.into()),
            );
        }
    }
}

#[post("/list", data = "<arg>")]
pub async fn list_dir(arg: Json<ListArg>) -> (Status, ListResponse) {
    let err_ret = |err: TinyDfsError| {
        let (status, etype, einfo) = err.exception();
        (
            status,
            ListResponse::ErrResp(
                ErrResponse {
                    exception_info: einfo.to_string(),
                    exception_type: etype.to_string(),
                }
                .into(),
            ),
        )
    };
    let res = dir_tree::lookup(&arg.path).await;
    if res.is_err() {
        return err_ret(TinyDfsError::PathInvalid);
    }
    let (_, target) = res.unwrap();
    if let Some(dir) = target {
        let files = dir.list().await;
        (
            Status::Ok,
            ListResponse::OkResp(ListOkResponse { files }.into()),
        )
    } else {
        err_ret(TinyDfsError::FileNotFound)
    }
}

#[post("/is_directory", data = "<arg>")]
pub async fn is_directory(arg: Json<IsDirectoryArg>) -> (Status, IsDirectoryResponse) {
    let err_ret = |err: TinyDfsError| {
        let (status, etype, einfo) = err.exception();
        (
            status,
            IsDirectoryResponse::ErrResp(
                ErrResponse {
                    exception_info: einfo.to_string(),
                    exception_type: etype.to_string(),
                }
                .into(),
            ),
        )
    };
    let res = dir_tree::lookup(&arg.path).await;
    if res.is_err() {
        return err_ret(TinyDfsError::PathInvalid);
    }
    let (_, target) = res.unwrap();
    if let Some(target) = target {
        let is_dir: bool;
        match target.as_ref() {
            dir_tree::File::RegFile(_) => is_dir = false,
            dir_tree::File::Dir(_) => is_dir = true,
        };
        (
            Status::Ok,
            IsDirectoryResponse::OkResp(OkResponse { success: is_dir }.into()),
        )
    } else {
        err_ret(TinyDfsError::FileNotFound)
    }
}
