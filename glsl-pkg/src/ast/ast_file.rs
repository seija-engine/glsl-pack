use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use anyhow::Result;
use glsl_lang::ast::ExternalDeclaration;
use crate::ast::ASTPackage;
use crate::package::{PackageInfo, Package};

use super::ast_package::PkgPath;
use super::{ SymbolName};
use super::errors::LoadFileError;
use super::scan_define::ScanDefine;
use super::scan_use::ScanUse;
#[derive(Debug)]
pub struct ASTFile {
    pub pkg_path:Arc<PkgPath>,
    pkg_info:Arc<PackageInfo>,
    pub path:String,
    uses:Vec<PkgPath>,
    pub defines:HashMap<String,ExternalDeclaration>
}

impl ASTFile {
    pub fn load<P:AsRef<Path>>(path:P,pkg_info:Arc<PackageInfo>) -> Result<ASTFile> {
       
        let code_string = std::fs::read_to_string(path.as_ref()).map_err(LoadFileError::IOError)?;
        Self::load_string(code_string,path.as_ref().to_str().unwrap(),pkg_info)
    }

    pub fn load_string(code:String,path:&str,pkg_info:Arc<PackageInfo>) -> Result<ASTFile> {
       
        let mut scan = ScanUse::new(&code);
        let (mut uses,remain) = scan.scan();
        for use_pkg in uses.iter_mut() {
            use_pkg.trim_left(&pkg_info.name);
        }
        let mut scan_define = ScanDefine::new(&remain)?;
        scan_define.scan();

        let pkg_path:Vec<String> = path.trim_end_matches(".glsl").split('/').map(String::from).collect();
        Ok(ASTFile {
            pkg_path:Arc::new(pkg_path.into()),
            pkg_info,
            path:String::from(path),
            uses,
            defines:scan_define.defines
        })
    }

    pub fn find_local_decl(&self,name:&str) -> Option<&ExternalDeclaration> {
        self.defines.get(name)
    }

    pub fn find_local_sym(&self,name:&str) -> Option<SymbolName> {
        if self.defines.contains_key(name) {
           return Some(self.pkg_path.to_sym(name));
        }
        None
    }

    pub fn find_sym(&self,name:&str,pkg:&ASTPackage) -> Option<SymbolName> {
        if let Some(v) = self.find_local_sym(name) {
            return Some(v);
        }
        for use_pkg in self.uses.iter() {
            if let Some(file)  = pkg.files.get(use_pkg) {
                if let Some(v) = file.find_local_sym(name) {
                    return Some(v);
                }
            }
        }
        None
    }
    /* 
    pub fn find_sym<'a,'b:'a>(&'a self,sym_name:&str,pkg:&'b ASTPackage) -> Option<&ExternalDeclaration> {
       if let Some(s) = self.defines.get(sym_name) {
           return Some(s);
       }
       for use_pkg in self.uses.iter() {
          if let Some(file)  = pkg.files.get(use_pkg) {
               let find:Option<&ExternalDeclaration> = file.find_sym(sym_name, pkg);
               if find.is_some() {
                   return find;
               }
          } else {
              log::error!("not find use:{:?}",use_pkg);
          }
       }
       None
    }
    */
}

