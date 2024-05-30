use std::{collections::BTreeMap, sync::Arc};

use once_cell::sync::Lazy;
use rocket::tokio::sync::Mutex;

use super::{error::NamingError, Ip};

pub struct StorageServer {
    ip: Ip,
    client_port: u16,
    command_port: u16,
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
    servers: BTreeMap<Ip, Arc<StorageServer>>,
}

impl ServerManager {
    pub fn new() -> Self {
        Self {
            servers: BTreeMap::new(),
        }
    }

    fn register_server(&mut self, srv: Arc<StorageServer>) -> Result<(), NamingError> {
        if self.servers.contains_key(&srv.ip) {
            Err(NamingError::ServerExists)
        } else {
            self.servers.insert(srv.ip.clone(), srv);
            Ok(())
        }
    }
}

static SERVER_MANAGER: Lazy<Mutex<ServerManager>> = Lazy::new(|| Mutex::new(ServerManager::new()));

pub async fn register_server(srv: Arc<StorageServer>) -> Result<(), NamingError> {
    SERVER_MANAGER.lock().await.register_server(srv)
}
