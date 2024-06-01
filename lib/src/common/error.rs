#[derive(Debug)]
pub enum TinyDfsError {
    StorageServerExists,
    FileExists,
    DirNotFound,
    DirReadErr,
    RegisterFailed,
    // TODO
}
