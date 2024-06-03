use std::{collections::BTreeMap, sync::Arc};

use once_cell::sync::Lazy;
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
    servers: BTreeMap<Ip, Arc<StorageServer>>,
}

impl ServerManager {
    pub fn new() -> Self {
        Self {
            servers: BTreeMap::new(),
        }
    }

    fn register_server(&mut self, srv: &Arc<StorageServer>) -> Result<(), TinyDfsError> {
        if self.servers.contains_key(&srv.ip) {
            Err(TinyDfsError::StorageServerExists)
        } else {
            self.servers.insert(srv.ip.clone(), srv.clone());
            Ok(())
        }
    }
}

static SERVER_MANAGER: Lazy<Mutex<ServerManager>> = Lazy::new(|| Mutex::new(ServerManager::new()));

pub async fn register_server(srv: &Arc<StorageServer>) -> Result<(), TinyDfsError> {
    SERVER_MANAGER.lock().await.register_server(srv)
}
