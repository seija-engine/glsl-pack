use thiserror::Error;

#[derive(Debug,Error)]
pub enum LoadFileError {
    #[error("{0}")]
    IOError(std::io::Error)
}