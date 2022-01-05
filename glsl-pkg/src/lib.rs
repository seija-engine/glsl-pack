use std::{path::Path, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use walkdir::DirEntry;

/*
step 1.
  读取AST到ASTPackage，同时把use信息读取到ASTFile上
step 2.
  CompileTask
  
*/
mod ast;
mod package;
mod shader;
mod errors;
mod compiler;
mod pkg_inst;

pub use compiler::{DepSearchGen};

#[derive(Debug,Default,Hash,PartialEq, Eq,Clone)]
pub struct MacroGroup {
  names:Vec<String>
}

impl MacroGroup {
  pub fn new(mut names:Vec<String>) -> Self {
    names.sort();
    MacroGroup { names }
  }

  pub fn hash_base64(&self) -> String {
    let mut hasher = DefaultHasher::default();
    self.names.hash(&mut hasher);
    let num = hasher.finish();
    let bytes = num.to_be_bytes();
    base64::encode(&bytes)
  }
}



pub fn walk_glsl_folder<P:AsRef<Path>>(path:P) -> impl Iterator<Item = DirEntry> {
  let wp = walkdir::WalkDir::new(path);
  wp.into_iter().filter_map(Result::ok).filter(is_glsl_file)                            
}

pub fn is_glsl_file(e:&DirEntry) -> bool {
  if e.file_type().is_file() == false { return  false; }
  e.file_name().to_str().map(|s| s.ends_with(".glsl")).unwrap_or(false)
}
/*
    Package {
        Shaders {
           name:String,
           vertInfo:Map<String,bool>,
           backend:Vec<String>,
           vs:String,
           fs:String,
           slots:Vec<String>
        }
    }

    根据宏组合的Cache?
*/