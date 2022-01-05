use std::collections::HashMap;

use anyhow::{Result};
use glsl_lang::ast::ExternalDeclaration;

use super::{ASTFile, SymbolName};
#[derive(Default,Debug)]
pub struct ASTPackage {
    name:String,
    files:HashMap<PkgPath,ASTFile>
}

impl ASTPackage {
    pub fn load_file(&mut self,name:String,path:&str,code_string:String) -> Result<()> {
        let ast_file = ASTFile::load_string(code_string)?;
        let pkg_path:Vec<String> = path.trim_end_matches(".glsl").split('/').map(String::from).collect();
        self.files.insert(pkg_path.into(), ast_file);
        self.name = name;
        Ok(())
    }

    pub fn find_sym(&self,sym:&SymbolName) -> Option<&ExternalDeclaration> {
        for (pkg_path,file) in self.files.iter() {
            if sym.quals.len() > 0 && sym.quals[0] == self.name {
                if sym.quals[1..] == pkg_path.paths {
                    return file.find_sym(&sym.name)
                }
            }
            else if sym.quals == pkg_path.paths {
                return file.find_sym(&sym.name)
            }
        }
        None
    }
}

#[derive(Hash,PartialEq, Eq,Debug)]
pub struct PkgPath {
    paths:Vec<String>
}

impl Into<PkgPath> for Vec<String> {
    fn into(self) -> PkgPath {
        PkgPath { paths:self }
    }
}