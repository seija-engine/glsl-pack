use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;
use glsl_lang::ast::ExternalDeclaration;
use super::{UseInfo, SymbolName};
use super::errors::LoadFileError;
use super::scan_define::ScanDefine;
use super::scan_use::ScanUse;
#[derive(Debug)]
pub struct ASTFile {
    uses:Vec<UseInfo>,
    pub defines:HashMap<String,ExternalDeclaration>
}

impl ASTFile {
    pub fn load<P:AsRef<Path>>(path:P) -> Result<ASTFile> {
        let code_string = std::fs::read_to_string(path).map_err(LoadFileError::IOError)?;
        Self::load_string(code_string)
    }

    pub fn load_string(code:String) -> Result<ASTFile> {
        let mut scan = ScanUse::new(&code);
        let (uses,remain) = scan.scan();
        let mut scan_define = ScanDefine::new(&remain)?;
        scan_define.scan();
        Ok(ASTFile {
            uses,
            defines:scan_define.defines
        })
    }

    pub fn find_sym(&self,sym_name:&str) -> Option<&ExternalDeclaration> {
        self.defines.get(sym_name)
    }
    
}


#[test]
fn test_file() {
    let f = ASTFile::load("../tests/core/math.glsl").unwrap();
    dbg!(f);
}