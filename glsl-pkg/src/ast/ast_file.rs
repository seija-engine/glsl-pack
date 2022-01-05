use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use anyhow::Result;
use glsl_lang::ast::ExternalDeclaration;
use crate::package::{PackageInfo, Package};

use super::{UseInfo, SymbolName};
use super::errors::LoadFileError;
use super::scan_define::ScanDefine;
use super::scan_use::ScanUse;
#[derive(Debug)]
pub struct ASTFile {
    pkg_info:Arc<PackageInfo>,
    path:String,
    uses:Vec<UseInfo>,
    pub defines:HashMap<String,ExternalDeclaration>
}

impl ASTFile {
    pub fn load<P:AsRef<Path>>(path:P,pkg_info:Arc<PackageInfo>) -> Result<ASTFile> {
        let code_string = std::fs::read_to_string(path.as_ref()).map_err(LoadFileError::IOError)?;
        Self::load_string(code_string,path.as_ref().to_str().unwrap(),pkg_info)
    }

    pub fn load_string(code:String,path:&str,pkg_info:Arc<PackageInfo>) -> Result<ASTFile> {
        let mut scan = ScanUse::new(&code);
        let (uses,remain) = scan.scan();
        let mut scan_define = ScanDefine::new(&remain)?;
        scan_define.scan();
        Ok(ASTFile {
            pkg_info,
            path:String::from(path),
            uses,
            defines:scan_define.defines
        })
    }

    pub fn find_sym(&self,sym_name:&str) -> Option<&ExternalDeclaration> {
       if let Some(s) = self.defines.get(sym_name) {
           return Some(s);
       }
       println!("full:{:?}",self.pkg_info.path);
       for use_info in self.uses.iter() {
           
       }
       println!("{:?} find:{}",self.path,sym_name);
       None
    }
    
}

