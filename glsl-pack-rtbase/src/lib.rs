use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use base58::ToBase58;
#[macro_use]
extern crate serde_derive;
pub mod shader;
pub mod rt_shaders;

#[derive(Debug,Default,Hash,PartialEq, Eq,Clone)]
pub struct MacroGroup {
   pub names:Vec<String>
}

impl MacroGroup {
  pub fn new(mut names:Vec<String>) -> Self {
    names.sort();
    MacroGroup { names }
  }

  pub fn join_to(&self,macros:Vec<String>) -> MacroGroup {
    let new_names = [self.names.clone(),macros].concat();
    MacroGroup::new(new_names)
  }

  pub fn hash_base64(&self) -> String {
    let mut hasher = DefaultHasher::default();
    self.names.hash(&mut hasher);
    let num = hasher.finish();
    let bytes = num.to_be_bytes();
    bytes.to_base58()
  }
}