use std::{collections::HashMap, sync::Arc};

use anyhow::{Result};
use glsl_lang::ast::ExternalDeclaration;

use crate::package::PackageInfo;

use super::{ASTFile, SymbolName};
#[derive(Default,Debug)]
pub struct ASTPackage {
    pub pkg_info:Arc<PackageInfo>,
    name:String,
    files:HashMap<PkgPath,Arc<ASTFile>>
}

impl ASTPackage {
    pub fn load_file(&mut self,name:String,path:&str,code_string:String) -> Result<()> {
        let ast_file = ASTFile::load_string(code_string,path,self.pkg_info.clone())?;
        let pkg_path:Vec<String> = path.trim_end_matches(".glsl").split('/').map(String::from).collect();
        self.files.insert(pkg_path.into(), Arc::new(ast_file));
        self.name = name;
        Ok(())
    }

    pub fn find_sym(&self,sym:&SymbolName) -> Option<(&ExternalDeclaration,Arc<ASTFile>)> {
        for (pkg_path,file) in self.files.iter() {
            if sym.quals.len() > 0 && sym.quals[0] == self.name {
                if sym.quals[1..] == pkg_path.paths {
                    return file.find_sym(&sym.name).map(|v| (v,file.clone()))
                }
            }
            else if sym.quals == pkg_path.paths {
                return file.find_sym(&sym.name).map(|v| (v,file.clone()))
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