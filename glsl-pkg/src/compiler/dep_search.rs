use std::collections::HashSet;
use std::sync::{Arc, Weak};
use glsl_lang::ast::*;
use crate::BACKENDS;
use crate::ast::{SymbolName, ASTFile, RcSymbolName};
use crate::buildin_sym::BuildinSymbols;
use crate::pkg_inst::PackageInstance;

pub struct DepSearch {
    pub symbols:Vec<RcSymbolName>,
    pub sets:HashSet<RcSymbolName>,
    scopes:Vec<SymbolScope>,
    buildin_syms:BuildinSymbols
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
        
        DepSearch {symbols:vec![], sets:HashSet::default(),scopes:Vec::new(),buildin_syms:BuildinSymbols::new(&BACKENDS) }
    }

    pub fn search(&mut self,sym:&SymbolName,pkg_inst:&PackageInstance) -> Vec<RcSymbolName> {
        self.search_symbol(sym,pkg_inst);

        self.scopes.clear();
        self.sets.clear();
        let mut outs = vec![];
        for ar_sym in self.symbols.drain(..) {
            outs.push(ar_sym);
        }
        outs
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
            self.push_last_scope(head_name.as_str());
        }
        self.search_type(&decl.content.head.content.ty.ty, file,pkg_inst);
        if let Some(initer) = decl.content.head.content.initializer.as_ref() {
            self.search_initializer(initer,file,pkg_inst);
        }

        for no_type_decl in decl.tail.iter() {
            let decl_name = no_type_decl.ident.ident.as_str();
            self.push_last_scope(decl_name);
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
                       if !self.buildin_syms.has_symbol(var.as_str()) {
                           log::warn!("not found var:{:?}",var)
                       }
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
            StatementData::CaseLabel(v) => {  log::warn!("error case:{:?}",&v); },
            StatementData::Iteration(iter) => {
                match &iter.content {
                    IterationStatementData::DoWhile(stmt,expr) => {
                        self.search_stmt_do_while(stmt,expr,file, pkg_inst);
                    },
                    IterationStatementData::While(cond,stmt) => {
                        self.search_stmt_while(cond,stmt, file, pkg_inst);
                    },
                    IterationStatementData::For(for_init,for_reset,stmt) => {
                        self.search_stmt_for(for_init, for_reset, stmt, file, pkg_inst);
                    }
                }
            },
            StatementData::Jump(jump) => {
                self.search_jump_stmt(jump,file,pkg_inst);
            },
            StatementData::Compound(v) => {
                self.enter_scope();
                for stmt in v.statement_list.iter() {
                    self.search_stmt(stmt, file, pkg_inst);
                }
                self.exit_scope();
            }
        }
    }

    fn search_jump_stmt(&mut self,stmt:&JumpStatement,file:&ASTFile,pkg_inst:&PackageInstance) {
        match &stmt.content {
            JumpStatementData::Return(expr) => {
                if let Some(re) = expr {
                    self.search_expr(re, file, pkg_inst);
                }
            },
            _ => {}
        }
    }

    fn search_stmt_for(&mut self,for_init:&ForInitStatement,for_reset:&ForRestStatement,stmt:&Statement,file:&ASTFile,pkg_inst:&PackageInstance) {
        self.enter_scope();
        match &for_init.content {
            ForInitStatementData::Expression(expr) => {
                if let Some(e) = expr {
                    self.search_expr(e, file, pkg_inst);
                }
            },
            ForInitStatementData::Declaration(decl) => {
                self.search_decl(decl, file, pkg_inst);
            }
        }

        if let Some(cond) = for_reset.condition.as_ref() {
            match &cond.content {
                ConditionData::Expr(e) => self.search_expr(e, file, pkg_inst),
                ConditionData::Assignment(full_type,id,initer) => {
                    self.search_type(&full_type.ty, file, pkg_inst);
                    self.push_last_scope(id.0.as_str());
                    self.search_initializer(initer,file,pkg_inst);
                }
            }
        }

        if let Some(e) = for_reset.post_expr.as_ref() {
            self.search_expr(e, file, pkg_inst);
        }
        match &stmt.content {
            StatementData::Compound(lst) => {
                for s in lst.statement_list.iter() {
                    self.search_stmt(s, file, pkg_inst)
                }
            },
            _ => self.search_stmt(stmt, file, pkg_inst)
        }
        self.exit_scope();
    }

    fn search_stmt_do_while(&mut self,stmt:&Statement,expr:&Expr,file:&ASTFile,pkg_inst:&PackageInstance) {
        self.search_stmt(stmt, file, pkg_inst);
        self.search_expr(expr, file, pkg_inst);
    }

    fn search_stmt_while(&mut self,cond:&Condition,stmt:&Statement,file:&ASTFile,pkg_inst:&PackageInstance) {
        self.enter_scope();
        match &cond.content {
            ConditionData::Assignment(full_type,id,initer) => {
                self.search_type(&full_type.ty, file, pkg_inst);
                self.push_last_scope(id.0.as_str());
                self.search_initializer(initer,file,pkg_inst);
            },
            ConditionData::Expr(expr) => {self.search_expr(expr, file, pkg_inst) }
        }
        match &stmt.content {
            StatementData::Compound(lst) => {
                for s in lst.statement_list.iter() {
                    self.search_stmt(s, file, pkg_inst)
                }
            },
            _ => self.search_stmt(stmt, file, pkg_inst)
        }
       
        self.exit_scope();
    }

    fn search_stmt_switch(&mut self,switch:&SwitchStatement,file:&ASTFile,pkg_inst:&PackageInstance) {
        self.search_expr(&switch.head, file, pkg_inst);
        for stmt in switch.body.iter() {
            match &stmt.content {
                StatementData::CaseLabel(_) => {},
                StatementData::Jump(_) => {},
                _ => {
                   self.search_stmt(stmt, file, pkg_inst);
                }
            }
            
        }
    }

    

    fn search_stmt_select(&mut self,select:&SelectionStatement,file:&ASTFile,pkg_inst:&PackageInstance) {
        self.search_expr(&select.cond, file, pkg_inst);
        match &select.rest.content {
            SelectionRestStatementData::Else(s1,s2) => {
               self.search_stmt(s1, file, pkg_inst);
               self.search_stmt(&s2, file, pkg_inst);
            },
            SelectionRestStatementData::Statement(s) => {
                self.search_stmt(&s, file, pkg_inst);
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
                None => {
                    if !self.buildin_syms.has_type(typ_name.0.as_str()) {
                        log::error!("not found type:{:?} {:?}: position:{:?}",typ_name.0,file.path,typ.span)
                    }
                } 
            }
        }
    }


    fn enter_function(&mut self,params:&Vec<FunctionParameterDeclaration>) {
        self.scopes.push(SymbolScope::default());
        for param in params.iter() {
            match &param.content {
                FunctionParameterDeclarationData::Named(_,v) => {
                    self.push_last_scope(v.content.ident.ident.0.as_str());
                },
                _ => {}   
            }
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(SymbolScope::default());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn push_last_scope(&mut self,name:&str) {
        if let Some(last_scope) = self.last_scope() {
            last_scope.push(name)
        }
    }

    fn last_scope(&mut self) -> Option<&mut SymbolScope> {
        self.scopes.last_mut()
    }

    fn add_search_sym(&mut self,sym:SymbolName) {
        let rc_sym:RcSymbolName = sym.into();
        if !self.sets.contains(&rc_sym) {
            self.sets.insert(rc_sym.clone());
            self.symbols.push(rc_sym);
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
