use std::{path::Path};

use walkdir::DirEntry;
mod ast;
mod package;
mod shader;
mod errors;
mod pkg_mgr;
mod compiler;
mod pkg_inst;
pub mod backends;
pub use pkg_mgr::{PackageManager};
pub use glsl_pack_rtbase::{MacroGroup};
pub use compiler::{CompileEnv,IShaderBackend};

#[macro_use]
extern crate lazy_static;

lazy_static! {
  pub static ref BACKENDS:backends::Backends = backends::Backends::new();
}


pub fn walk_glsl_folder<P:AsRef<Path>>(path:P) -> impl Iterator<Item = DirEntry> {
  let wp = walkdir::WalkDir::new(path);
  wp.into_iter().filter_map(Result::ok).filter(is_glsl_file)                            
}

pub fn is_glsl_file(e:&DirEntry) -> bool {
  if e.file_type().is_file() == false { return  false; }
  e.file_name().to_str().map(|s| s.ends_with(".glsl")).unwrap_or(false)
}