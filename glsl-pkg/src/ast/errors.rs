use thiserror::Error;

#[derive(Debug,Error)]
pub enum LoadPackageError {
    #[error("not found folder: {0}")]
    NotFoundFolder(String)
}

#[derive(Debug,Error)]
pub enum LoadFileError {
    #[error("{0}")]
    IOError(std::io::Error)
}