#[macro_use]
extern crate rocket;

pub mod common;
mod naming;
mod storage;

pub use naming::start_naming_server;
pub use storage::start_storage_server;
