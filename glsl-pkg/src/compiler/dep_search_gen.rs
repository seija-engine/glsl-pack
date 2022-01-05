use std::{sync::Arc, marker::PhantomData};
use std::fmt::Write;
use glsl_lang::ast::*;
use glsl_lang::transpiler::glsl::{self as glsl_t, FormattingState};

use crate::ast::ASTFile;
use crate::{pkg_inst::PackageInstance, ast::SymbolName};

use super::compile_env::CompileEnv;

pub struct DepSearchGen<F> {
    inst:Arc<PackageInstance>,
    _marker:PhantomData<F>,
}


impl<F> DepSearchGen<F> where F:Write {
    pub fn new(inst:Arc<PackageInstance>) -> Self {
       
        DepSearchGen { inst,_marker:PhantomData }
    }

    pub fn run(&mut self,fn_names:Vec<SymbolName>,write:&mut F,env:&mut CompileEnv) {
        let mut fs = glsl_t::FormattingState::default();
        for name in fn_names.iter() {
            self.import(name,write,&mut fs,env);
        }
    }

    fn import(&self,sym_name:&SymbolName,write:&mut F,fs:&mut FormattingState,env:&mut CompileEnv) {
       if let Some((ed,file)) = self.inst.ast_pkg.find_sym(sym_name) {
          self.import_edecl(ed, write,fs,env,&file);
       }
    }

    fn import_edecl(&self,decl:&ExternalDeclaration,write:&mut F,fs:&mut FormattingState,env:&mut CompileEnv,file:&ASTFile) {
        glsl_t::show_external_declaration(write, decl, fs).unwrap();
        match &decl.content {
            ExternalDeclarationData::Declaration(decl) => self.import_decl(decl, write, fs,env,file),
            ExternalDeclarationData::FunctionDefinition(fd) => self.import_fn_decl(fd,write, env,file),
            _ => {}
        }
    }

    fn import_decl(&self,decl:&Declaration,write:&mut F,fs:&mut FormattingState,env:&mut CompileEnv,file:&ASTFile) {
        match &decl.content {
            DeclarationData::InitDeclaratorList(init_decl) => {

            }
            DeclarationData::FunctionPrototype(fn_decl) => self.import_fn_prototype(fn_decl,env,write,file),
            DeclarationData::Block(block) => {},
            DeclarationData::Precision(_, _) => {},
        }
    }

    fn import_fn_decl(&self,fn_decl:&FunctionDefinition,write:&mut F,env:&mut CompileEnv,file:&ASTFile) {
        self.import_fn_prototype(&fn_decl.content.prototype, env,write,file);
    }

    fn import_fn_prototype(&self,fp:&FunctionPrototype,env:&mut CompileEnv,write:&mut F,file:&ASTFile) {
       self.import_type(&fp.content.ty, write, file);
    }


    //////////////////////////////////////////////
    fn import_type(&self,typ:&FullySpecifiedType,write:&mut F,file:&ASTFile) {
        if let TypeSpecifierNonArrayData::TypeName(typ_name) = &typ.ty.ty.content {
            file.find_sym(typ_name.0.as_str());
        }
    }
}