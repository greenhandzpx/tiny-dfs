use std::{collections::BTreeMap, future::Future, sync::Arc};

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

    pub fn for_all_servers<F, T>(&self, mut func: F) -> T
    where
        F: FnMut(&mut Vec<Arc<StorageServer>>) -> T,
    {
        match self {
            File::RegFile(f) => {
                let mut servers = f.srvs.lock().unwrap();
                func(&mut servers)
            }
            File::Dir(_) => panic!(),
        }
    }

    async fn lookup(&self, child: &str) -> Option<Arc<File>> {
        match self {
            File::RegFile(_) => None,
            File::Dir(f) => f.lookup(child).await,
        }
    }

    async fn delete_file(&self, child: &str) -> Option<Arc<File>> {
        match self {
            File::RegFile(_) => panic!(),
            File::Dir(f) => f.delete_file(child).await,
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
    /// Several servers may own this file
    srvs: std::sync::Mutex<Vec<Arc<StorageServer>>>,
    name: String,
}

impl RegFile {
    fn new(name: &str, srv: &Arc<StorageServer>) -> Self {
        Self {
            srvs: std::sync::Mutex::new(vec![srv.clone()]),
            name: name.to_string(),
        }
    }
}

pub struct Dir {
    children: Mutex<BTreeMap<String, Arc<File>>>,
    name: String,
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            children: Mutex::new(BTreeMap::new()),
            name: name.to_string(),
        }
    }

    async fn lookup(&self, child: &str) -> Option<Arc<File>> {
        self.children.lock().await.get(child).cloned()
    }

    async fn delete_file(&self, child: &str) -> Option<Arc<File>> {
        self.children.lock().await.remove(child)
    }

    async fn create_file(&self, child: &str, is_dir: bool, srv: Option<&Arc<StorageServer>>) {
        let file = if is_dir {
            Arc::new(File::Dir(Dir::new(child)))
        } else {
            Arc::new(File::RegFile(RegFile::new(child, srv.unwrap())))
        };
        self.children.lock().await.insert(child.to_string(), file);
    }
}

static ROOT_DIR: Lazy<Arc<File>> = Lazy::new(|| Arc::new(File::Dir(Dir::new("/"))));

#[derive(Default)]
struct WalkDirTreeOption {
    /// Auto create the missing intermediate ones
    create_inter_one: bool,
    /// Auto create the missing target
    create_target: bool,
    need_target_name: bool,
}

enum WalkDirTreeTarget {
    Some(Arc<File>),
    /// Target name
    Name(Option<String>),
}

impl WalkDirTreeTarget {
    fn from_file(file: Option<Arc<File>>, name: Option<String>) -> Self {
        if let Some(f) = file {
            Self::Some(f)
        } else {
            Self::Name(name)
        }
    }
}

impl From<WalkDirTreeTarget> for Option<Arc<File>> {
    fn from(value: WalkDirTreeTarget) -> Self {
        match value {
            WalkDirTreeTarget::Some(f) => Some(f),
            WalkDirTreeTarget::Name(_) => None,
        }
    }
}

/// cb: callback for parent dir and target file
async fn walk_dir_tree<F, Fut, T>(path: &str, option: WalkDirTreeOption, cb: F) -> T
where
    F: Fn(Option<Arc<File>>, WalkDirTreeTarget) -> Fut,
    Fut: Future<Output = T>,
{
    let split_path: Vec<&str> = path.split("/").collect();
    let mut parent_dir = ROOT_DIR.clone();

    for (i, name) in split_path.iter().enumerate() {
        let mut target = parent_dir.lookup(name).await;
        if target.is_some() {
            if i != split_path.len() - 1 {
                parent_dir = target.unwrap();
            } else {
                return cb(Some(parent_dir), WalkDirTreeTarget::from_file(target, None)).await;
            }
        } else {
            if i == split_path.len() - 1 {
                // Cannot find the target
                if option.create_target {
                    parent_dir.create_file(name, true, None).await;
                    target = parent_dir.lookup(name).await;
                }
                let name = if option.need_target_name {
                    Some(name.clone().to_owned())
                } else {
                    None
                };
                return cb(Some(parent_dir), WalkDirTreeTarget::from_file(None, name)).await;
            } else {
                // Cannot find the intermediate one
                if !option.create_inter_one {
                    log::info!("Dir {:?} in Path {:?} not found", name, path);
                    let name = if option.need_target_name {
                        Some(name.clone().to_owned())
                    } else {
                        None
                    };
                    return cb(None, WalkDirTreeTarget::from_file(None, name)).await;
                }
                parent_dir.create_file(name, true, None).await;
                parent_dir = parent_dir.lookup(name).await.unwrap();
            }
        }
    }
    unreachable!()
}

/// Return parent dir and target file (if any)
pub async fn lookup(path: &str) -> (Option<Arc<File>>, Option<Arc<File>>) {
    walk_dir_tree(
        path,
        WalkDirTreeOption::default(),
        |parent, target| async move { (parent, target.into()) },
    )
    .await
}

pub async fn delete_file(path: &str) -> Result<Arc<File>, TinyDfsError> {
    log::debug!("delete_file: path {:?}", path,);
    walk_dir_tree(
        path,
        WalkDirTreeOption::default(),
        |parent, target| async move {
            if let Some(parent) = parent {
                match target {
                    WalkDirTreeTarget::Some(target) => {
                        let child = parent.delete_file(target.name()).await;
                        return Ok(child.unwrap());
                    }
                    WalkDirTreeTarget::Name(_) => return Err(TinyDfsError::FileNotFound),
                }
            } else {
                log::warn!("delete_file: Path {:?} has missing one", path);
                return Err(TinyDfsError::DirNotFound);
            }
        },
    )
    .await
}

async fn create_file(
    path: &str,
    is_dir: bool,
    srv: Option<&Arc<StorageServer>>,
    create_missing_one: bool,
) -> Result<(), TinyDfsError> {
    log::debug!(
        "create_file: path {:?}, is_dir {:?}, auto_create {:?}",
        path,
        is_dir,
        create_missing_one
    );
    walk_dir_tree(
        path,
        WalkDirTreeOption {
            create_inter_one: true,
            create_target: false,
            need_target_name: true,
        },
        |parent, target| {
            async move {
                if let Some(parent) = parent {
                    match target {
                        WalkDirTreeTarget::Some(_) => {
                            // The new file has existed
                            log::warn!("Path {:?} has existed", path);
                            return Err(TinyDfsError::FileExists);
                        }
                        WalkDirTreeTarget::Name(name) => {
                            let name = name.unwrap();
                            parent.create_file(&name, is_dir, srv).await;
                            return Ok(());
                        }
                    }
                } else {
                    log::warn!("create_file: Path {:?} has missing one", path);
                    return Err(TinyDfsError::DirNotFound);
                }
            }
        },
    )
    .await
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
