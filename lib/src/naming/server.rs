use std::sync::Arc;

use once_cell::sync::Lazy;
use rand::Rng;
use rocket::tokio::sync::Mutex;

use crate::common::error::TinyDfsError;

use super::Ip;

pub struct StorageServer {
    pub ip: Ip,
    pub client_port: u16,
    pub command_port: u16,
}

impl StorageServer {
    pub fn new(ip: Ip, client_port: u16, command_port: u16) -> Self {
        Self {
            ip,
            client_port,
            command_port,
        }
    }
}

struct ServerManager {
    // servers: BTreeMap<Ip, Arc<StorageServer>>,
    servers: Vec<Arc<StorageServer>>,
}

impl ServerManager {
    fn new() -> Self {
        Self {
            servers: Vec::new(),
        }
    }

    fn register_server(&mut self, srv: &Arc<StorageServer>) -> Result<(), TinyDfsError> {
        if self.servers.iter().any(|s| s.ip == srv.ip) {
            Err(TinyDfsError::StorageServerExists)
        } else {
            Ok(self.servers.push(srv.clone()))
        }
    }

    // fn get(&self, idx: usize) -> Option<Arc<StorageServer>> {
    //     self.servers.get(idx).cloned()
    // }

    fn get_random(&self) -> Option<Arc<StorageServer>> {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.servers.len());
        self.servers.get(idx).cloned()
    }
}

static SERVER_MANAGER: Lazy<Mutex<ServerManager>> = Lazy::new(|| Mutex::new(ServerManager::new()));

pub async fn register_server(srv: &Arc<StorageServer>) -> Result<(), TinyDfsError> {
    SERVER_MANAGER.lock().await.register_server(srv)
}

pub async fn select_random_server() -> Option<Arc<StorageServer>> {
    SERVER_MANAGER.lock().await.get_random()
}
