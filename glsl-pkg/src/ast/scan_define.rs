use std::collections::HashMap;

use anyhow::{Result};
use glsl_lang::ast::{TranslationUnit, ExternalDeclaration, ExternalDeclarationData, Node, Declaration, DeclarationData, InitDeclaratorList, TypeSpecifierNonArray, TypeSpecifierNonArrayData};
use glsl_lang::parse::{Parsable, ParseContext, ParseContextData};

pub struct ScanDefine {
    unit:Option<TranslationUnit>,
    pub defines:HashMap<String,ExternalDeclaration>
}

impl ScanDefine {
    pub fn new(code:&str) -> Result<ScanDefine> {
        let mut opts = ParseContext::default();
        opts.opts.skip_type_check = true;
        
        let (unit,_) = TranslationUnit::parse_with_options(code, &opts)?;
        Ok(ScanDefine { unit:Some(unit),defines:HashMap::default() })
    }

    pub fn scan(&mut self) {
       let exprs:Vec<ExternalDeclaration> = self.unit.take().unwrap().0;
       for expr in exprs {
           match &expr.content {
               ExternalDeclarationData::Declaration(decl) => {
                   match &decl.content {
                    DeclarationData::InitDeclaratorList(init_lst) => {
                        let ty = &init_lst.head.ty.ty.ty;
                        match &ty.content {
                            TypeSpecifierNonArrayData::Struct(s) => {
                                if let Some(n) = s.name.as_ref() {
                                    self.defines.insert(n.0.to_string(), expr);
                                }
                            },
                            _ => {
                                if let Some(n) = init_lst.head.name.as_ref() {
                                    let name = n.0.to_string();
                                    self.defines.insert(name, expr);
                                }
                            }
                        }
                    }
                    _ => {}
                   }
               },
               ExternalDeclarationData::FunctionDefinition(fn_decl) => {
                   let name = fn_decl.prototype.name.0.to_string();
                   self.defines.insert(name, expr);
               }
               _ => {}
           } 
       }
    }
}