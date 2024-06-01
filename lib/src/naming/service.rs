use rocket::{http::Status, serde::json::Json};

use crate::{
    common::service::{IsValidPathArg, IsValidPathResponse},
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
