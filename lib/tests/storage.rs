use tiny_dfs::common::{service::{CreateDirectoryArg, CreateFileArg, DeleteArg, GetStorageArg, GetStorageOkResponse, IsValidPathArg, IsValidPathResponse}, storage::{base64_decode, base64_encode, ReadArg, ReadOkResponse, WriteArg}, ErrResponse, OkResponse};

mod common;

#[rocket::tokio::test(flavor = "multi_thread")]
async fn test_read_write() {
    let service_port = 11111;
    let client_port = 33333;
    // env_logger::try_init();
    let new_files = vec!["/test111", "/test222"];

    common::init(&new_files).await;
    let client = reqwest::Client::new();

    log::warn!("test_create_file: start...");
    let create_dir = "/test886";
    let create_file = "/test886/test888";


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


    log::info!("start to verify path...");
    // Verify that the file has been deleted in the naming server
    let arg = IsValidPathArg {
        path: create_file.to_string(),
    };
    let addr = format!("http://localhost:{}/is_valid_path", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    assert!(resp.status().is_success());
    let resp: IsValidPathResponse = resp.json().await.unwrap();
    assert!(resp.success);


    log::info!("start to get storage...");
    let arg = GetStorageArg {
        path: create_file.to_string(),
    };
    let addr = format!("http://localhost:{}/getstorage", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    assert!(resp.status().is_success());
    let resp: GetStorageOkResponse = resp.json().await.unwrap();
    assert!(resp.server_port == client_port);
    let storage_port = resp.server_port;


    log::info!("start to write file...");
    let data = "hello world!!!";
    let encoded = base64_encode(data);
    let arg = WriteArg {
        path: create_file.to_string(),
        offset: 0,
        data: encoded.clone(),
    };
    let addr = format!("http://localhost:{}/storage_write", storage_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    if !resp.status().is_success() {
        let resp: ErrResponse = resp.json().await.unwrap();
        log::error!("resp err, exception info: {}", resp.exception_info);
        panic!();
    };
    assert!(resp.status().is_success());
    let resp: OkResponse = resp.json().await.unwrap();
    assert!(resp.success);


    log::info!("start to read file...");
    let arg = ReadArg {
        path: create_file.to_string(),
        offset: 0,
        length: data.len() as i32,
    };
    let addr = format!("http://localhost:{}/storage_read", storage_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    if !resp.status().is_success() {
        let resp: ErrResponse = resp.json().await.unwrap();
        log::error!("resp err, exception info: {}", resp.exception_info);
        panic!();
    };
    assert!(resp.status().is_success());
    let resp: ReadOkResponse = resp.json().await.unwrap();
    assert!(resp.data.eq(&encoded));
    let decoded = base64_decode(resp.data).unwrap();
    log::info!("read data {:?}", String::from_utf8(decoded).unwrap());


}
