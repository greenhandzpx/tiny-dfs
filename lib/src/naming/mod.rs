//! Code of naming server

mod api;
mod dir_tree;
mod server;

use api::registration::register_storage_server;
use api::service::{
    create_directory, create_file, delete_file, get_storage_server, is_directory, is_valid_path,
    list_dir,
};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ip(pub String);

/// args[2]: service port;
/// args[3]: registration port
pub async fn start_naming_server(args: &Vec<String>) {
    log::info!("start a new naming server...");
    let service_port = args[2].parse::<u16>().unwrap();
    let registration_port = args[3].parse::<u16>().unwrap();

    let service_config = rocket::Config {
        port: service_port,
        ..rocket::Config::debug_default()
    };
    let registration_config = rocket::Config {
        port: registration_port,
        ..rocket::Config::debug_default()
    };

    let service_task = rocket::tokio::spawn(async move {
        rocket::build()
            .configure(service_config)
            .mount(
                "/",
                routes![
                    is_valid_path,
                    get_storage_server,
                    delete_file,
                    create_directory,
                    create_file,
                    list_dir,
                    is_directory,
                ],
            )
            // .mount("/test", routes![hello])
            .launch()
            .await
            .unwrap();
    });

    let registration_task = rocket::tokio::spawn(async move {
        rocket::build()
            .configure(registration_config)
            .mount("/", routes![register_storage_server])
            .launch()
            .await
            .unwrap();
    });

    service_task.await.unwrap();
    registration_task.await.unwrap();
}
