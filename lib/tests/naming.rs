use std::{fs, time::Duration, vec};

use tiny_dfs::{
    common::service::{IsValidPathArg, IsValidPathResponse},
    start_naming_server, start_storage_server,
};
use tokio::time::sleep;

#[rocket::tokio::test(flavor = "multi_thread")]
async fn test_valid_path() {
    env_logger::init();

    let service_port = "11111";
    let registration_port = "22222";

    // Start a naming server
    let naming_server = rocket::tokio::spawn(async move {
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
    let new_file = "/test111";
    fs::File::create(local_dir.to_owned() + new_file).unwrap();
    let storage_server = rocket::tokio::spawn(async move {
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

    log::info!("start to verify path...");
    // Verify that the newly created file exists in the naming server
    let arg = IsValidPathArg {
        path: new_file.to_string(),
    };
    let client = reqwest::Client::new();
    let addr = format!("http://localhost:{}/is_valid_path", service_port);
    let resp = client.post(addr).json(&arg).send().await.unwrap();
    assert!(resp.status().is_success());
    let resp: IsValidPathResponse = resp.json().await.unwrap();
    assert!(resp.success);
}
