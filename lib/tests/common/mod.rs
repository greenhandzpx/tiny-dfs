use std::{fs, time::Duration};

use once_cell::sync::Lazy;
use rocket::futures::lock::Mutex;
use tiny_dfs::{start_naming_server, start_storage_server};
use tokio::time::sleep;

static INIT_LOCK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub async fn init(new_files: &Vec<&str>) {
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
