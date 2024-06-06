use std::vec;

use tiny_dfs::common::{
    service::{
        CreateDirectoryArg, CreateFileArg, DeleteArg, IsDirectoryArg, IsValidPathArg,
        IsValidPathResponse, ListArg, ListOkResponse,
    },
    ErrResponse, OkResponse,
};

mod common;

#[rocket::tokio::test(flavor = "multi_thread")]
async fn test_valid_path() {
    let new_files = vec!["/test111", "/test222"];

    common::init(&new_files).await;

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

    common::init(&new_files).await;

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

    common::init(&new_files).await;

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

    log::info!("start to create dir...");
    let arg = CreateDirectoryArg {
        path: create_dir.to_string(),
    };
    let client = reqwest::Client::new();
    let addr = format!("http://localhost:{}/create_directory", service_port);
    let _resp = client.post(addr).json(&arg).send().await.unwrap();

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

#[rocket::tokio::test(flavor = "multi_thread")]
async fn test_list_dir() {
    let service_port = 11111;
    // env_logger::try_init();
    let new_files = vec!["/test111", "/test222"];

    common::init(&new_files).await;

    log::warn!("test_create_file: start...");
    let create_dir = "/test886";
    let create_file = "/test886/test888";
    let create_file2 = "/test886/test777";
    let client = reqwest::Client::new();

    log::info!("start to delete file...");
    let arg = DeleteArg {
        path: create_file.to_string(),
    };
    let addr = format!("http://localhost:{}/delete", service_port);
    let _resp = client.post(addr).json(&arg).send().await.unwrap();

    log::info!("start to create dir...");
    let arg = CreateDirectoryArg {
        path: create_dir.to_string(),
    };
    let addr = format!("http://localhost:{}/create_directory", service_port);
    let _resp = client.post(addr).json(&arg).send().await.unwrap();

    log::info!("start to create file...");
    let arg = CreateFileArg {
        path: create_file.to_string(),
    };
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

    log::info!("start to create file2...");
    let arg = CreateFileArg {
        path: create_file2.to_string(),
    };
    let addr = format!("http://localhost:{}/create_file", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    assert!(resp.status().is_success());
    let resp: OkResponse = resp.json().await.unwrap();
    assert!(resp.success);

    log::info!("start to test is_dir...");
    let arg = IsDirectoryArg {
        path: create_dir.to_string(),
    };
    let addr = format!("http://localhost:{}/is_directory", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    assert!(resp.status().is_success());
    let resp: OkResponse = resp.json().await.unwrap();
    assert!(resp.success);

    log::info!("start to test list dir...");
    let arg = ListArg {
        path: create_dir.to_string(),
    };
    let addr = format!("http://localhost:{}/list", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    assert!(resp.status().is_success());
    let resp: ListOkResponse = resp.json().await.unwrap();
    log::info!("list dir: {:?}", resp.files);
}
