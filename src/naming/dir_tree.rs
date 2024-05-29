use std::sync::Arc;

use once_cell::sync::Lazy;
use rocket::tokio::sync::Mutex;


struct Server {
    ip: String,
    client_port: u16,
    command_port: u16,
}

enum File {
    RegFile(RegFile),
    Dir(Dir),
}

struct RegFile {
    srv: Arc<Server>,
    name: String,
}

struct Dir {
    children: Vec<File>,
}

static root_dir: Lazy<Mutex<Option<Dir>>> = Lazy::new(|| Mutex::new(None));