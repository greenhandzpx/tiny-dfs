use rocket::http::Status;

#[derive(Debug, PartialEq, Eq)]
pub enum TinyDfsError {
    StorageServerExists,
    FileExists,
    FileNotFound,
    DirNotFound,
    DirReadErr,
    RegisterFailed,
    PathInvalid,
    // TODO
}

impl TinyDfsError {
    pub fn exception(&self) -> (Status, &'static str, &'static str) {
        match self {
            TinyDfsError::StorageServerExists => (
                Status::Conflict,
                "IllegalStateException",
                "storage server exists",
            ),
            TinyDfsError::FileExists => (Status::Conflict, "IllegalStateException", "file exists"),
            TinyDfsError::FileNotFound => {
                (Status::NotFound, "FileNotFoundException", "file not found")
            }
            TinyDfsError::DirNotFound => (
                Status::NotFound,
                "FileNotFoundException",
                "(parent) dir not found",
            ),
            TinyDfsError::DirReadErr => (Status::Conflict, "IllegalStateException", "dir read err"),
            TinyDfsError::RegisterFailed => (
                Status::Conflict,
                "IllegalStateException",
                "server registerred failed",
            ),
            TinyDfsError::PathInvalid => {
                (Status::NotFound, "IllegalArgumentException", "path invalid")
            }
        }
    }
}
