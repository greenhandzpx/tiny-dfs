#[macro_use]
extern crate rocket;

use env_logger;
use naming::register_storage_server;

mod naming;
mod storage;

// #[get("/<name>/<age>")]
// fn hello(name: &str, age: u8) -> String {
//     format!("Hello, {} year old named {}!", age, name)
// }
async fn start_naming_server(args: &Vec<String>) {
    if args.len() != 4 {
        panic!();
    }
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
            .mount("/register", routes![register_storage_server]).launch().await.unwrap();
    });

    let registration_task = rocket::tokio::spawn(async move {
        rocket::build()
            .configure(registration_config)
            .mount("/test", routes![register_storage_server]).launch().await.unwrap();
    });

    service_task.await.unwrap();
    registration_task.await.unwrap();

    // rocket::build()
    //     .configure(registration_config)
    //     .mount("/register", routes![register_storage_server]).launch().await.unwrap();
}

async fn start_storage_server(args: &Vec<String>) {
    todo!()
}

#[rocket::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        log::error!("Invalid arg. Usage: cargo run [server_type(naming/storage)] ...");
        panic!();
    }

    let server_type: &str = &args[1];
    if server_type == "naming" {
        start_naming_server(&args).await;
    } else if server_type == "storage" {
        start_storage_server(&args).await;
    } else {
        log::error!("Unknown server type");
        panic!();
    }
}
