use std::fs;

use rocket::{http::Status, serde::json::Json};

use crate::{
    common::{
        error::ErrResponse,
        service::{DeleteArg, DeleteOkResponse, DeleteResponse},
    },
    storage::path,
};

#[post("/storage_delete", data = "<arg>")]
pub fn delete_file(arg: Json<DeleteArg>) -> (Status, DeleteResponse) {
    let global_path: &str = &arg.path;
    let local_path = path::global_to_local(global_path);

    log::info!("delete_file: local path {:?}", local_path);
    if fs::remove_file(local_path).is_ok() {
        (
            Status::Ok,
            DeleteResponse::OkResp(DeleteOkResponse { success: true }.into()),
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
