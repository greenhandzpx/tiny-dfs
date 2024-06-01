use std::sync::Arc;

use once_cell::sync::Lazy;
use rocket::tokio::sync::Mutex;

use crate::common::error::TinyDfsError;

use super::server::StorageServer;

pub enum File {
    RegFile(RegFile),
    Dir(Dir),
}

impl File {
    fn name(&self) -> &str {
        match self {
            File::RegFile(f) => &f.name,
            File::Dir(f) => &f.name,
        }
    }

    async fn lookup(&self, child: &str) -> Option<Arc<File>> {
        match self {
            File::RegFile(_) => None,
            File::Dir(f) => f.lookup(child).await,
        }
    }

    async fn create_file(&self, child: &str, is_dir: bool, srv: Option<&Arc<StorageServer>>) {
        match self {
            File::RegFile(_) => panic!(),
            File::Dir(f) => f.create_file(child, is_dir, srv).await,
        }
    }
}

pub struct RegFile {
    srv: Arc<StorageServer>,
    name: String,
}

impl RegFile {
    fn new(name: &str, srv: &Arc<StorageServer>) -> Self {
        Self {
            srv: srv.clone(),
            name: name.to_string(),
        }
    }
}

pub struct Dir {
    children: Mutex<Vec<Arc<File>>>,
    name: String,
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            children: Mutex::new(Vec::new()),
            name: name.to_string(),
        }
    }

    async fn lookup(&self, child: &str) -> Option<Arc<File>> {
        let mut matched: Vec<Arc<File>> = self
            .children
            .lock()
            .await
            .iter()
            .filter(|file| file.name() == child)
            .map(|file| file.clone())
            .collect();
        assert!(matched.len() <= 1);
        matched.pop()
    }

    async fn create_file(&self, child: &str, is_dir: bool, srv: Option<&Arc<StorageServer>>) {
        let file = if is_dir {
            Arc::new(File::Dir(Dir::new(child)))
        } else {
            Arc::new(File::RegFile(RegFile::new(child, srv.unwrap())))
        };
        self.children.lock().await.push(file);
    }
}

static ROOT_DIR: Lazy<Arc<File>> = Lazy::new(|| Arc::new(File::Dir(Dir::new("/"))));

/// Return parent dir and target file (if any)
pub async fn lookup(path: &str) -> (Option<Arc<File>>, Option<Arc<File>>) {
    let split_path: Vec<&str> = path.split("/").collect();
    let mut parent_dir = ROOT_DIR.clone();

    for (i, name) in split_path.iter().enumerate() {
        let target = parent_dir.lookup(name).await;
        if target.is_some() {
            if i != split_path.len() - 1 {
                parent_dir = target.unwrap();
            } else {
                return (Some(parent_dir), target);
           }
        } else {
            if i == split_path.len() - 1 {
                return (Some(parent_dir), None);
            } else {
                return (None, None);
            }
        }
    }
    panic!()
}

async fn create_file(
    path: &str,
    is_dir: bool,
    srv: Option<&Arc<StorageServer>>,
    auto_create: bool,
) -> Result<(), TinyDfsError> {
    let split_path: Vec<&str> = path.split("/").collect();
    let mut parent_dir = ROOT_DIR.clone();

    log::debug!(
        "path {:?}, is_dir {:?}, auto_create {:?}",
        path,
        is_dir,
        auto_create
    );
    for (i, name) in split_path.iter().enumerate() {
        let target = parent_dir.lookup(name).await;
        if target.is_some() {
            if i != split_path.len() - 1 {
                parent_dir = target.unwrap();
            } else {
                // The new file has existed
                log::warn!("Path {:?} has existed", path);
                return Err(TinyDfsError::FileExists);
            }
        } else {
            if i == split_path.len() - 1 {
                parent_dir.create_file(name, is_dir, srv).await;
                return Ok(());
            } else {
                // Cannot find the intermediate one
                if !auto_create {
                    log::warn!("Dir {:?} in Path {:?} not found", name, path);
                    return Err(TinyDfsError::DirNotFound);
                }
                parent_dir.create_file(name, true, None).await;
                parent_dir = parent_dir.lookup(name).await.unwrap();
            }
        }
    }
    panic!()
}

/// Collect necessary files and retrive all duplicated ones
pub async fn collect_files(
    files: &Vec<String>,
    srv: &Arc<StorageServer>,
) -> Result<Vec<String>, TinyDfsError> {
    let mut duplicated_files: Vec<String> = Vec::new();
    for file in files {
        let (_, target) = lookup(&file).await;
        if target.is_some() {
            duplicated_files.push(file.clone());
            continue;
        }
        create_file(&file, false, Some(srv), true).await?;
    }
    Ok(duplicated_files)
}
