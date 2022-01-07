use std::collections::HashSet;
use std::sync::Arc;
use glsl_lang::ast::*;
use crate::ast::{SymbolName, ASTFile};
use crate::pkg_inst::PackageInstance;

pub struct DepSearch {
    pub symbols:Vec<Arc<SymbolName>>,
    pub sets:HashSet<Arc<SymbolName>>,
    scopes:Vec<SymbolScope>,
}

#[derive(Default,Debug)]
struct SymbolScope {
    syms:HashSet<String>
}

impl SymbolScope {
    pub fn push(&mut self,name:&str) {
        self.syms.insert(name.to_string());
    }

    pub fn has(&self,name:&str) -> bool {
        self.syms.contains(name)
    }
}

impl DepSearch {
    pub fn new() -> Self {
        DepSearch {symbols:vec![], sets:HashSet::default(),scopes:Vec::new() }
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
        if let Some(head_name) = decl.head.name.as_ref() {
            self.last_scope().push(head_name.as_str());
        }
        self.search_type(&decl.content.head.content.ty.ty, file,pkg_inst);
        if let Some(initer) = decl.content.head.content.initializer.as_ref() {
            self.search_initializer(initer,file,pkg_inst);
        }

        for no_type_decl in decl.tail.iter() {
            let decl_name = no_type_decl.ident.ident.as_str();
            self.last_scope().push(decl_name);
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
            ExprData::Variable(var) => {
               if !self.has_sym(var.as_str()) {
                   if let Some(find_sym) = file.find_sym(var.as_str(), &pkg_inst.ast_pkg) {
                        self.add_search_sym(find_sym);
                   } else {
                       log::error!("not found var:{:?}",var)
                   }
               }
            },
            ExprData::Unary(_,e) => self.search_expr(e, file, pkg_inst),
            ExprData::Binary(_,a,b) => {
                self.search_expr(&a,file, pkg_inst);
                self.search_expr(&b,file, pkg_inst);
            }
            ExprData::Ternary(a,b,c) => {
                self.search_expr(&a,file, pkg_inst);
                self.search_expr(&b,file, pkg_inst);
                self.search_expr(&c,file, pkg_inst);
            },
            ExprData::Assignment(lv,_,rv) => {
                self.search_expr(&lv,file, pkg_inst);
                self.search_expr(&rv,file, pkg_inst);
            },
            ExprData::Bracket(a,b) => {
                self.search_expr(&a,file, pkg_inst);
                self.search_expr(&b,file, pkg_inst);
            },
            ExprData::FunCall(id,exprs) => {
                self.search_fun_identifier(id, file,pkg_inst);
                for v in exprs.iter() {
                    self.search_expr(v, file,pkg_inst);
                }
            },
            ExprData::PostInc(e) => self.search_expr(e, file, pkg_inst),
            ExprData::PostDec(e) => self.search_expr(e, file, pkg_inst),
            ExprData::Comma(a,b) => {
                self.search_expr(&a,file, pkg_inst);
                self.search_expr(&b,file, pkg_inst);
            }
            ExprData::Dot(a,_id) => {
                self.search_expr(&a,file, pkg_inst);
            }
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
        self.enter_function(&fn_decl.prototype.parameters);
        self.search_fn_prototype(&fn_decl.content.prototype,file,pkg_inst);
        for stmt in  fn_decl.content.statement.content.statement_list.iter() {
            self.search_stmt(stmt, file,pkg_inst);
        }
        self.exit_scope();
    }

    fn search_stmt(&mut self,stmt:&Statement,file:&ASTFile,pkg_inst:&PackageInstance) {
        match &stmt.content {
            StatementData::Declaration(decl) => {
                self.search_decl(decl, file,pkg_inst);
            }
            StatementData::Expression(expr) => {
                if let Some(v) = expr.0.as_ref() {
                    self.search_expr(v, file,pkg_inst)
                }
            },
            StatementData::Selection(select) => {
                self.search_stmt_select(select,file,pkg_inst);
            }
            StatementData::Switch(switch) => {
                self.search_stmt_switch(switch, file, pkg_inst);
            },
            StatementData::CaseLabel(v) => {
                match &v.content {
                    CaseLabelData::Case(expr) => {
                        self.search_expr(&expr, file, pkg_inst);
                    },
                    _ => {}
                }
            },
            StatementData::Iteration(iter) => {
                match &iter.content {
                    IterationStatementData::DoWhile(stmt,expr) => {
                        self.search_stmt(stmt, file, pkg_inst);
                        self.search_expr(expr, file, pkg_inst);
                    },
                    IterationStatementData::While(cond,stmt) => {
                        self.search_stmt(stmt, file, pkg_inst);
                    }
                    _ => {}
                }
            },
            StatementData::Compound(v) => {
                for stmt in v.statement_list.iter() {
                    self.search_stmt(stmt, file, pkg_inst);
                }
            }
            _ => {}
        }
    }

    

    fn search_stmt_switch(&mut self,switch:&SwitchStatement,file:&ASTFile,pkg_inst:&PackageInstance) {
        self.search_expr(&switch.head, file, pkg_inst);
        for stmt in switch.body.iter() {
            
            self.search_stmt(stmt, file, pkg_inst);
        }
    }

    

    fn search_stmt_select(&mut self,select:&SelectionStatement,file:&ASTFile,pkg_inst:&PackageInstance) {
        self.search_expr(&select.cond, file, pkg_inst);
        match &select.rest.content {
            SelectionRestStatementData::Else(s1,s2) => {
                
                //self.search_stmt(s1, file, pkg_inst);
                
                //self.search_stmt(s2, file, pkg_inst);
            },
            SelectionRestStatementData::Statement(s) => {
                self.enter_scope();
                self.search_stmt(s, file, pkg_inst);
                self.exit_scope();
            }
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
                    self.add_search_sym(sym);
                },
                None => log::error!("not found type:{:?} {:?}: position:{:?}",typ_name.0,file.path,typ.span)
            }
        }
    }


    fn enter_function(&mut self,params:&Vec<FunctionParameterDeclaration>) {
        self.scopes.push(SymbolScope::default());
        for param in params.iter() {
            match &param.content {
                FunctionParameterDeclarationData::Named(_,v) => {
                    self.last_scope().push(v.content.ident.ident.0.as_str());
                },
                _ => {}   
            }
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(SymbolScope::default());
    }

    fn exit_scope(&mut self) {
        dbg!(&self.scopes);
        self.scopes.pop();
    }

    fn last_scope(&mut self) -> &mut SymbolScope {
        self.scopes.last_mut().unwrap()
    }

    fn add_search_sym(&mut self,sym:SymbolName) {
        if !self.sets.contains(&sym) {
            let s = Arc::new(sym);
            self.sets.insert(s.clone());
            self.symbols.push(s);
        }
    }

    fn has_sym(&self,name:&str) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.has(name) {
                return true;
            }
        }
        false
    }
}


#[test]
fn load_package() {
    use crate::package::Package;
    use crate::MacroGroup;

    env_logger::init();
    let mut pkg = Package::load("../tests/core/").unwrap();
    let macros = &MacroGroup::new(vec!["HAS_POSITION".to_string()]);
    let sym_vs = SymbolName::parse("core.color.vs_main");
    //let sym_fs = SymbolName::parse("core.color.fs_main");


    let inst = pkg.get_inst(macros);
    let mut dep_search = DepSearch::new();
    dep_search.search(vec![sym_vs],&inst);

    dbg!(&dep_search.symbols);

}