#[macro_use]
extern crate rocket;

use env_logger;

mod common;
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

    naming::init(args).await;
}

async fn start_storage_server(args: &Vec<String>) {
    if args.len() != 6 {
        panic!();
    }

    storage::init(args).await;
    // todo!()
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
