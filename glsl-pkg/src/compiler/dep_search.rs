use std::collections::HashSet;

use glsl_lang::ast::*;
use crate::MacroGroup;
use crate::ast::{SymbolName, ASTFile};
use crate::package::Package;
use crate::pkg_inst::PackageInstance;

pub struct DepSearch {
    pub search_symbols:HashSet<SymbolName>
}

impl DepSearch {
    pub fn new() -> Self {
        DepSearch { search_symbols:HashSet::default() }
    }

    pub fn search(&mut self,syms:Vec<SymbolName>,pkg_inst:&PackageInstance) {
        
        for name in syms.iter() {
            self.search_symbol(name,pkg_inst);
        }
    }

    fn search_symbol(&mut self,sym_name:&SymbolName,pkg_inst:&PackageInstance) {
        if let Some((ed,file)) = pkg_inst.ast_pkg.find_decl(sym_name) {
            self.search_edecl(ed, &file,pkg_inst);
        }
    }

    fn search_edecl(&mut self,decl:&ExternalDeclaration,file:&ASTFile,pkg_inst:&PackageInstance) {
        match &decl.content {
            ExternalDeclarationData::Declaration(decl) => self.search_decl(decl,file,pkg_inst),
            ExternalDeclarationData::FunctionDefinition(fd) => self.search_fn_decl(fd,file,pkg_inst),
            _ => {}
        }
    }

    fn search_decl(&mut self,decl:&Declaration,file:&ASTFile,pkg_inst:&PackageInstance) {
        match &decl.content {
            DeclarationData::InitDeclaratorList(init_decl) => self.search_init_decl(init_decl,file,pkg_inst),
            DeclarationData::FunctionPrototype(fn_decl) => self.search_fn_prototype(fn_decl,file,pkg_inst),
            DeclarationData::Block(block) => {},
            DeclarationData::Precision(_, _) => {},
        }
    }

    fn search_init_decl(&mut self,decl:&InitDeclaratorList,file:&ASTFile,pkg_inst:&PackageInstance) {
        self.search_type(&decl.content.head.content.ty.ty, file,pkg_inst);
        if let Some(initer) = decl.content.head.content.initializer.as_ref() {
            self.search_initializer(initer,file,pkg_inst);
        }
    }

    fn search_initializer(&mut self,initer:&Initializer,file:&ASTFile,pkg_inst:&PackageInstance) {
        match &initer.content {
            InitializerData::List(lst) => {
               for v in lst.iter() {
                self.search_initializer(v,file,pkg_inst);
               }
            },
            InitializerData::Simple(v) => {
                self.search_expr(v, file,pkg_inst);
            }
        }
    }

    fn search_expr(&mut self,expr:&Expr,file:&ASTFile,pkg_inst:&PackageInstance) {
      
        match &expr.content {
            ExprData::FunCall(id,exprs) => {
                self.search_fun_identifier(id, file,pkg_inst);
                for v in exprs.iter() {
                    self.search_expr(v, file,pkg_inst);
                }
            },
            ExprData::Variable(var) => {
               
            }
            ExprData::Assignment(v,_,_) => {

            },
            _ => {}
        }
    }

    fn search_fun_identifier(&mut self,fn_id:&FunIdentifier,file:&ASTFile,pkg_inst:&PackageInstance) {
        match &fn_id.content {
            FunIdentifierData::Expr(expr) => {
                self.search_expr(expr, file,pkg_inst);
            },
            FunIdentifierData::TypeSpecifier(ty) => {
                self.search_type(ty, file,pkg_inst);
            }
        }
    }

    

    fn search_fn_decl(&mut self,fn_decl:&FunctionDefinition,file:&ASTFile,pkg_inst:&PackageInstance) {
        self.search_fn_prototype(&fn_decl.content.prototype,file,pkg_inst);
        for stmt in  fn_decl.content.statement.content.statement_list.iter() {
            self.search_stmt(stmt, file,pkg_inst);
        }
    }

    fn search_stmt(&mut self,stmt:&Statement,file:&ASTFile,pkg_inst:&PackageInstance) {
        match &stmt.content {
            StatementData::CaseLabel(v) => {},
            StatementData::Declaration(decl) => {
                self.search_decl(decl, file,pkg_inst);
            }
            StatementData::Expression(expr) => {
                if let Some(v) = expr.0.as_ref() {
                    self.search_expr(v, file,pkg_inst)
                }
            },
            _ => {}
        }
    }

    fn search_fn_prototype(&mut self,fp:&FunctionPrototype,file:&ASTFile,pkg_inst:&PackageInstance) {
       self.search_type(&fp.content.ty.ty, file,pkg_inst);
       for param in fp.content.parameters.iter() {
              match &param.content {
                  FunctionParameterDeclarationData::Named(_,v) => {
                      self.search_type(&v.ty, file,pkg_inst);
                  },
                  FunctionParameterDeclarationData::Unnamed(_,v) => {
                    self.search_type(&v, file,pkg_inst);
                  }
              }
       }
    }


    fn search_type(&mut self,typ:&TypeSpecifier,file:&ASTFile,pkg_inst:&PackageInstance) {
        if let TypeSpecifierNonArrayData::TypeName(typ_name) = &typ.ty.content {
            match file.find_sym(typ_name.0.as_str(),&pkg_inst.ast_pkg) {
                Some(sym) => {
                    self.search_symbols.insert(sym);
                },
                None => log::error!("not found type:{:?} {:?}: position:{:?}",typ_name.0,file.path,typ.span)
            }
        }
    }
}


#[test]
fn load_package() {
    env_logger::init();
    let mut pkg = Package::load("../tests/core/").unwrap();
    let macros = &MacroGroup::new(vec!["HAS_POSITION".to_string()]);
    let sym_vs = SymbolName::parse("core.color.vs_main");
    //let sym_fs = SymbolName::parse("core.color.fs_main");


    let inst = pkg.get_inst(macros);
    let mut dep_search = DepSearch::new();
    dep_search.search(vec![sym_vs],&inst);

    dbg!(&dep_search.search_symbols);

}