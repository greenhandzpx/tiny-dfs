use std::sync::Arc;

use once_cell::sync::Lazy;
use rocket::tokio::sync::Mutex;

use super::server::StorageServer;

enum File {
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
            File::Dir(f) => f.lookup(child).await
        }
    }
}

struct RegFile {
    srv: Arc<StorageServer>,
    name: String,
}

struct Dir {
    children: Mutex<Vec<Arc<File>>>,
    name: String,
}

impl Dir {
    fn new(name: String) -> Self {
        Self {
            children: Mutex::new(Vec::new()),
            name,
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
}

static ROOT_DIR: Lazy<Arc<File>> = Lazy::new(|| Arc::new(File::Dir(Dir::new("/".to_string()))));

/// Return parent dir and target file (if any)
async fn lookup(path: String) -> (Option<Arc<File>>, Option<Arc<File>>) {

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
                return (Some(parent_dir), None)
            } else {
                return (None, None)
            }
        }
    }
    panic!()
}

fn retrive_duplicated_files(files: Vec<String>) -> Vec<String> {
    todo!()
}

pub fn collect_files(files: Vec<String>) -> Vec<String> {
    
    todo!()
}
