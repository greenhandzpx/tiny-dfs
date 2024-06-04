use std::fs;

use rocket::{http::Status, serde::json::Json};

use crate::{
    common::{
        service::{CreateFileArg, CreateFileResponse, DeleteArg, DeleteResponse},
        ErrResponse, OkResponse,
    },
    storage::path,
};

#[post("/storage_delete", data = "<arg>")]
pub fn delete_file(arg: Json<DeleteArg>) -> (Status, DeleteResponse) {
    let global_path: &str = &arg.path;
    let local_path = path::global_to_local(global_path);

    log::info!("delete_file: local path {:?}", local_path);
    // todo!("delete empty directory");
    if fs::remove_file(local_path).is_ok() {
        (
            Status::Ok,
            DeleteResponse::OkResp(OkResponse { success: true }.into()),
        )
    } else {
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

#[post("/storage_create", data = "<arg>")]
pub fn create_file(arg: Json<CreateFileArg>) -> (Status, CreateFileResponse) {
    let global_path: &str = &arg.path;
    let local_path = path::global_to_local(global_path);

    log::info!("create_file: local path {:?}", local_path);
    // Also create the missing intermediate ones
    let local_path = std::path::Path::new(&local_path);
    if let Some(parent_dir) = local_path.parent() {
        fs::create_dir_all(parent_dir).unwrap();
    }
    if fs::File::create(local_path).is_ok() {
        (
            Status::Ok,
            CreateFileResponse::OkResp(OkResponse { success: true }.into()),
        )
    } else {
        (
            Status::NotFound,
            CreateFileResponse::ErrResp(
                ErrResponse {
                    exception_type: "IllegalArgumentException".into(),
                    exception_info: "IllegalArgumentException: path invalid.".into(),
                }
                .into(),
            ),
        )
    }
}
