use std::{collections::HashMap, sync::Arc};

use anyhow::{Result};
use glsl_lang::ast::ExternalDeclaration;

use crate::package::PackageInfo;

use super::{ASTFile, SymbolName};
#[derive(Default,Debug)]
pub struct ASTPackage {
    pub pkg_info:Arc<PackageInfo>,
    name:String,
    pub files:HashMap<Arc<PkgPath>,Arc<ASTFile>>
}

impl ASTPackage {
    pub fn load_file(&mut self,name:String,path:&str,code_string:String) -> Result<()> {
        let ast_file = ASTFile::load_string(code_string,path,self.pkg_info.clone())?;
        self.files.insert(ast_file.pkg_path.clone(), Arc::new(ast_file));
        self.name = name;
        Ok(())
    }

    
    pub fn find_decl(&self,sym:&SymbolName) -> Option<(&ExternalDeclaration,Arc<ASTFile>)> {
        for (pkg_path,file) in self.files.iter() {
            if sym.quals.len() > 0 && sym.quals[0] == self.name {
                if sym.quals[1..] == pkg_path.paths {
                    return file.find_local_decl(&sym.name).map(|v| (v,file.clone()))
                }
            }
            else if sym.quals == pkg_path.paths {
                return file.find_local_decl(&sym.name).map(|v| (v,file.clone()))
            }
        }
        None
    }
}

#[derive(Hash,PartialEq, Eq,Debug)]
pub struct PkgPath {
    paths:Vec<String>
}

impl PkgPath {
    pub fn from_string(str:&str) -> Self {
        let paths = str.split('.').map(String::from).collect();
        PkgPath { paths }
    }

    pub fn trim_left(&mut self,name:&str) {
        if self.paths[0] == name {
            self.paths.remove(0);
        }
    }

    pub fn to_sym(&self,name:&str) -> SymbolName {
        SymbolName { quals: self.paths.clone(), name: name.to_string() }
    }
}

impl Into<PkgPath> for Vec<String> {
    fn into(self) -> PkgPath {
        PkgPath { paths:self }
    }
}