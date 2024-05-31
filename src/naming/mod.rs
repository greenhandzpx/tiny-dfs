//! Code of naming server

mod dir_tree;
mod registration;
mod server;
mod service;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Ip(String);

use registration::register_storage_server;

pub async fn init(args: &Vec<String>) {
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
