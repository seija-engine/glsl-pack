use serde_json::Value;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum PackageLoadError {
    #[error("not found package json")]
    NotFoundPackageJson,
    #[error("json error {0}")]
    JsonError(&'static str)
}


#[derive(Debug,Error)]
pub enum ShaderLoadError {
    #[error("json error {0}")]
    JsonError(&'static str)
}