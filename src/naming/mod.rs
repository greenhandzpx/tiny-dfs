mod dir_tree;
mod error;
mod registration;
mod server;
mod service;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Ip(String);

pub use registration::register_storage_server;