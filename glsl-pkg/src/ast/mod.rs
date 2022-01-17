mod ast_file;
mod ast_package;
mod scan_use;
mod scan_define;
mod lex_string;
mod errors;
use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

pub use ast_file::ASTFile;
pub use ast_package::ASTPackage;

#[derive(Debug,PartialEq, Default,Eq,Hash,Clone)]
pub struct SymbolName {
    quals:Vec<String>,
    name:String
}

impl SymbolName {
    pub fn parse(str:&str) -> Self {
        let mut cur_name = String::default();
        let mut names:Vec<String> = vec![];
        for chr in str.chars() {
            if chr == '.' {
                names.push(cur_name);
                cur_name = String::default();
            } else {
                cur_name.push(chr);
            }
        }

        SymbolName { quals: names, name: cur_name }
    }

    pub fn hash_u64(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
