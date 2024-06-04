use std::{fs, time::Duration, vec};

use once_cell::sync::Lazy;
use rocket::futures::lock::Mutex;
use tiny_dfs::{
    common::{
        service::{
            CreateDirectoryArg, CreateFileArg, DeleteArg, IsValidPathArg, IsValidPathResponse,
        },
        ErrResponse, OkResponse,
    },
    start_naming_server, start_storage_server,
};
use tokio::time::sleep;

static INIT_LOCK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

async fn init(new_files: &Vec<&str>) {
    let mut guard = INIT_LOCK.lock().await;
    println!("=================== start to init... ======================== ");
    if *guard {
        println!("=================== somebody has init ======================== ");
        return;
    }
    *guard = true;

    env_logger::init();

    let service_port = "11111";
    let registration_port = "22222";

    // Start a naming server
    let _naming_server = rocket::tokio::spawn(async move {
        let args = vec![
            "".to_string(),
            "".to_string(),
            service_port.to_string(),
            registration_port.to_string(),
        ];
        start_naming_server(&args).await;
    });

    // Start a storage server
    let local_dir = "/tmp/tiny-dfs";
    // let new_file = "/test111";
    for new_file in new_files {
        fs::File::create(local_dir.to_owned() + new_file).unwrap();
    }
    let _storage_server = rocket::tokio::spawn(async move {
        let args = vec![
            "".to_string(),
            "".to_string(),
            "33333".to_string(),
            "44444".to_string(),
            registration_port.to_string(),
            local_dir.to_string(),
        ];
        start_storage_server(&args).await;
    });

    sleep(Duration::from_millis(300)).await;
}

#[rocket::tokio::test(flavor = "multi_thread")]
async fn test_valid_path() {
    let new_files = vec!["/test111", "/test222"];

    init(&new_files).await;

    log::warn!("test_verify_path: start...");

    log::info!("start to verify path...");
    // Verify that the newly created file exists in the naming server
    let arg = IsValidPathArg {
        path: new_files[0].to_string(),
    };
    let client = reqwest::Client::new();
    let addr = format!("http://localhost:{}/is_valid_path", 11111);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    assert!(resp.status().is_success());
    let resp: IsValidPathResponse = resp.json().await.unwrap();
    assert!(resp.success);
}

#[rocket::tokio::test(flavor = "multi_thread")]
async fn test_delete_file() {
    let service_port = 11111;
    // env_logger::try_init();
    let new_files = vec!["/test111", "/test222"];

    init(&new_files).await;

    log::warn!("test_delete_file: start...");

    log::info!("start to verify path 1...");
    // Verify that the newly created file exists in the naming server
    let arg = IsValidPathArg {
        path: new_files[1].to_string(),
    };
    let client = reqwest::Client::new();
    let addr = format!("http://localhost:{}/is_valid_path", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    assert!(resp.status().is_success());
    let resp: IsValidPathResponse = resp.json().await.unwrap();
    assert!(resp.success);

    log::info!("start to delete file...");
    let arg = DeleteArg {
        path: new_files[1].to_string(),
    };
    let client = reqwest::Client::new();
    let addr = format!("http://localhost:{}/delete", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    // assert!(resp.status().is_success());
    if !resp.status().is_success() {
        log::error!("test_delete_file: resp status {:?}", resp.status());
        panic!();
    }
    let resp: OkResponse = resp.json().await.unwrap();
    assert!(resp.success);

    log::info!("start to verify path 2...");
    // Verify that the file has been deleted in the naming server
    let arg = IsValidPathArg {
        path: new_files[1].to_string(),
    };
    let client = reqwest::Client::new();
    let addr = format!("http://localhost:{}/is_valid_path", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    assert!(resp.status().is_success());
    let resp: IsValidPathResponse = resp.json().await.unwrap();
    assert!(!resp.success);
}

#[rocket::tokio::test(flavor = "multi_thread")]
async fn test_create_file() {
    let service_port = 11111;
    // env_logger::try_init();
    let new_files = vec!["/test111", "/test222"];

    init(&new_files).await;

    log::warn!("test_create_file: start...");
    let create_dir = "/test886";
    let create_file = "/test886/test888";

    log::info!("start to delete file...");
    let arg = DeleteArg {
        path: create_file.to_string(),
    };
    let client = reqwest::Client::new();
    let addr = format!("http://localhost:{}/delete", service_port);
    let _resp = client.post(addr).json(&arg).send().await.unwrap();
    // if !resp.status().is_success() {
    //     log::error!("test_delete_file: resp status {:?}", resp.status());
    //     panic!();
    // }
    // let resp: OkResponse = resp.json().await.unwrap();
    // assert!(resp.success);

    log::info!("start to create dir...");
    let arg = CreateDirectoryArg {
        path: create_dir.to_string(),
    };
    let client = reqwest::Client::new();
    let addr = format!("http://localhost:{}/create_directory", service_port);
    let _resp = client.post(addr).json(&arg).send().await.unwrap();
    // if !resp.status().is_success() {
    //     log::error!("test_create_file: resp status {:?}", resp.status());
    //     let resp: ErrResponse = resp.json().await.unwrap();
    //     log::error!("exception info: {}", resp.exception_info);
    //     panic!();
    // }
    // let resp: OkResponse = resp.json().await.unwrap();
    // assert!(resp.success);

    log::info!("start to create file...");
    let arg = CreateFileArg {
        path: create_file.to_string(),
    };
    let client = reqwest::Client::new();
    let addr = format!("http://localhost:{}/create_file", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    // assert!(resp.status().is_success());
    if !resp.status().is_success() {
        log::error!("test_create_file: resp status {:?}", resp.status());
        let resp: ErrResponse = resp.json().await.unwrap();
        log::error!("exception info: {}", resp.exception_info);
        panic!();
    }
    let resp: OkResponse = resp.json().await.unwrap();
    assert!(resp.success);

    log::info!("start to verify path...");
    // Verify that the file has been deleted in the naming server
    let arg = IsValidPathArg {
        path: create_file.to_string(),
    };
    let client = reqwest::Client::new();
    let addr = format!("http://localhost:{}/is_valid_path", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    assert!(resp.status().is_success());
    let resp: IsValidPathResponse = resp.json().await.unwrap();
    assert!(resp.success);
}
