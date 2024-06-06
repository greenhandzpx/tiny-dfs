use std::{
    fs,
    io::{ErrorKind, Read, Seek, SeekFrom, Write},
};

use rocket::{http::Status, serde::json::Json};

use crate::{
    common::{
        error::TinyDfsError,
        storage::{
            base64_decode, base64_encode, ReadArg, ReadOkResponse, ReadResponse, SizeArg,
            SizeOkResponse, SizeResponse, WriteArg, WriteResponse,
        },
        ErrResponse, OkResponse,
    },
    storage::path::{self, path_is_invalid},
};

#[post("/storage_size", data = "<arg>")]
pub fn get_size(arg: Json<SizeArg>) -> (Status, SizeResponse) {
    let err_ret = |err: TinyDfsError| {
        let (status, etype, einfo) = err.exception();
        (
            status,
            SizeResponse::ErrResp(
                ErrResponse {
                    exception_info: einfo.to_string(),
                    exception_type: etype.to_string(),
                }
                .into(),
            ),
        )
    };
    let global_path: &str = &arg.path;
    if path_is_invalid(global_path) {
        return err_ret(TinyDfsError::PathInvalid);
    }
    let local_path = path::global_to_local(global_path);

    log::info!("get_size: local path {:?}", local_path);

    let metadata = fs::metadata(local_path);
    if let Some(metadata) = metadata.ok() {
        (
            Status::Ok,
            SizeResponse::OkResp(
                SizeOkResponse {
                    size: metadata.len(),
                }
                .into(),
            ),
        )
    } else {
        err_ret(TinyDfsError::FileNotFound)
    }
}

#[post("/storage_read", data = "<arg>")]
pub fn read_file(arg: Json<ReadArg>) -> (Status, ReadResponse) {
    let err_ret = |err: TinyDfsError| {
        let (status, etype, einfo) = err.exception();
        (
            status,
            ReadResponse::ErrResp(
                ErrResponse {
                    exception_info: einfo.to_string(),
                    exception_type: etype.to_string(),
                }
                .into(),
            ),
        )
    };

    let global_path: &str = &arg.path;
    if path_is_invalid(global_path) {
        return err_ret(TinyDfsError::PathInvalid);
    }
    let local_path = path::global_to_local(global_path);

    log::info!("read_file: local path {:?}", local_path);

    if arg.length < 0 {
        return err_ret(TinyDfsError::IndexOutOfBound);
    }
    let file = fs::File::open(local_path);
    if file.is_err() {
        return err_ret(TinyDfsError::FileNotFound);
    }
    let mut file = file.unwrap();
    if file.seek(SeekFrom::Start(arg.offset)).is_err() {
        return err_ret(TinyDfsError::IndexOutOfBound);
    }
    let mut buf = vec![0; arg.length as usize];
    if let Some(err) = file.read_exact(&mut buf).err() {
        let resp_err = match err.kind() {
            ErrorKind::UnexpectedEof => TinyDfsError::IndexOutOfBound,
            ErrorKind::Interrupted => TinyDfsError::IOInterrupted,
            _ => TinyDfsError::FileNotFound,
        };
        return err_ret(resp_err);
    } else {
        let encoded = base64_encode(buf);
        (
            Status::Ok,
            ReadResponse::OkResp(ReadOkResponse { data: encoded }.into()),
        )
    }
}

#[post("/storage_write", data = "<arg>")]
pub fn write_file(arg: Json<WriteArg>) -> (Status, WriteResponse) {
    let err_ret = |err: TinyDfsError| {
        let (status, etype, einfo) = err.exception();
        (
            status,
            WriteResponse::ErrResp(
                ErrResponse {
                    exception_info: einfo.to_string(),
                    exception_type: etype.to_string(),
                }
                .into(),
            ),
        )
    };

    let global_path: &str = &arg.path;
    if path_is_invalid(global_path) {
        return err_ret(TinyDfsError::PathInvalid);
    }
    let local_path = path::global_to_local(global_path);

    log::info!("write_file: local path {:?}", local_path);

    let file = fs::OpenOptions::new().read(true).write(true).open(local_path);
    if file.is_err() {
        log::warn!("write_file:{}: file not found", line!());
        return err_ret(TinyDfsError::FileNotFound);
    }
    let mut file = file.unwrap();
    if file.seek(SeekFrom::Start(arg.offset)).is_err() {
        log::warn!("write_file:{}: seek failed", line!());
        return err_ret(TinyDfsError::IndexOutOfBound);
    }
    let decoded = base64_decode(&arg.data).unwrap();
    if let Some(err) = file.write_all(&decoded).err() {
        let resp_err = match err.kind() {
            ErrorKind::UnexpectedEof => TinyDfsError::IndexOutOfBound,
            ErrorKind::Interrupted => TinyDfsError::IOInterrupted,
            _ => TinyDfsError::FileNotFound,
        };
        log::warn!("write_file:{}: write err, kind {:?}", line!(), err.kind());
        return err_ret(resp_err);
    } else {
        (
            Status::Ok,
            WriteResponse::OkResp(OkResponse { success: true }.into()),
        )
    }
}
