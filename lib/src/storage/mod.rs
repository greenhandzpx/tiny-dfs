//! Code of storage server

mod command;
mod path;

use std::{
    fs,
    sync::atomic::{AtomicU16, Ordering},
};

use command::delete_file;
use once_cell::sync::Lazy;

use crate::common::{
    error::TinyDfsError,
    registration::{RegisterArg, RegisterOkResponse},
};

static CLIENT_PORT: Lazy<AtomicU16> = Lazy::new(|| AtomicU16::new(0));
static COMMAND_PORT: Lazy<AtomicU16> = Lazy::new(|| AtomicU16::new(0));

const SERVER_IP: &str = "localhost";
const NAMING_SERVER_IP: &str = "localhost";

async fn regsiter_myself(registration_port: u16) -> Result<(), TinyDfsError> {
    // Collect all local files
    let mut files: Vec<String> = Vec::new();
    let local_dir = path::global_to_local("/");
    let entries = fs::read_dir(local_dir.as_str()).or(Err(TinyDfsError::DirReadErr))?;

    for entry in entries {
        let path = entry.or(Err(TinyDfsError::DirReadErr))?.path();
        log::debug!("{}: send path {:?}", line!(), path.as_os_str());
        // Convert to local path
        let global_path = path::local_to_global(path.to_str().unwrap());
        // let relative_path = &path.to_str().unwrap().to_string()[local_dir.len()..];
        log::debug!("{}: send relative path {:?}", line!(), global_path);
        files.push(global_path.to_string());
    }

    let arg = RegisterArg {
        storage_ip: SERVER_IP.to_string(),
        client_port: CLIENT_PORT.load(Ordering::Relaxed),
        command_port: COMMAND_PORT.load(Ordering::Relaxed),
        files,
    };

    // Send registration request
    let client = reqwest::Client::new();
    let addr = format!("http://{}:{}/register", NAMING_SERVER_IP, registration_port);
    log::debug!("addr {:?}", addr);
    let resp = client
        .post(addr)
        .json(&arg)
        .send()
        .await
        .or(Err(TinyDfsError::RegisterFailed))?;

    if !resp.status().is_success() {
        log::warn!("{}: status {:?}", line!(), resp.status());
        return Err(TinyDfsError::RegisterFailed);
    }
    let resp: RegisterOkResponse = resp.json().await.unwrap();

    // Remove duplicated files
    let duplicated_files = resp.files;
    for file in duplicated_files {
        log::info!("{}: remove relative path {}", line!(), file);
        // let path = local_dir.to_string() + &file;
        let path = path::global_to_local(&file);
        log::info!("{}: remove path {}", line!(), path);
        fs::remove_file(file).unwrap();
    }
    Ok(())
}

/// args: Command line args;
/// args[2]: client port;
/// args[3]: command port;
/// args[4]: regsitration port (in naming server);
/// args[5]: local dir
pub async fn start_storage_server(args: &Vec<String>) {
    log::info!("start a new storage server...");

    let client_port = args[2].parse::<u16>().unwrap();
    let command_port = args[3].parse::<u16>().unwrap();
    let registration_port = args[4].parse::<u16>().unwrap();
    let local_dir = args[5].clone();

    CLIENT_PORT.store(client_port, Ordering::Relaxed);
    COMMAND_PORT.store(command_port, Ordering::Relaxed);
    path::set_local_dir(local_dir);
    // *path::local_dir().write().await = local_dir;

    if let Some(err) = regsiter_myself(registration_port).await.err() {
        log::error!("register failed, err {:?}", err);
        panic!();
    }

    let client_config = rocket::Config {
        port: client_port,
        ..rocket::Config::debug_default()
    };
    let command_config = rocket::Config {
        port: command_port,
        ..rocket::Config::debug_default()
    };

    let client_task = rocket::tokio::spawn(async move {
        rocket::build()
            .configure(client_config)
            .launch()
            .await
            .unwrap();
    });

    let command_task = rocket::tokio::spawn(async move {
        rocket::build()
            .configure(command_config)
            .mount("/", routes![delete_file])
            .launch()
            .await
            .unwrap();
    });

    client_task.await.unwrap();
    command_task.await.unwrap();
}
