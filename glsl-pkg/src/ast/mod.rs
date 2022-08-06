mod ast_file;
mod ast_package;
mod scan_use;
mod scan_define;
mod lex_string;
mod errors;
use std::{ hash::{Hash}, sync::Arc};

pub use ast_file::ASTFile;
pub use ast_package::ASTPackage;

#[derive(Debug,PartialEq, Default,Eq,Hash,Clone)]
pub struct SymbolName {
    quals:Vec<String>,
    pub name:String
}
#[derive(Debug,PartialEq, Default,Eq,Hash,Clone)]
pub struct RcSymbolName(pub Arc<SymbolName>);

impl Into<RcSymbolName> for SymbolName {
    fn into(self) -> RcSymbolName {
        RcSymbolName(Arc::new(self))
    }
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
}
