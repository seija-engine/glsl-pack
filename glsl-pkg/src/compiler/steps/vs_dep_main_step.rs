use std::{fmt::Write, sync::Arc};

use glsl_lang::{ast::*, transpiler::glsl::show_function_definition, visitor::*};

use crate::{shader::Shader, compiler::sym_generator::SymbolGenerator, pkg_inst::PackageInstance, ast::SymbolName};

pub fn run_vs_dep_main_step<W:Write>(_shader:&Shader,main_name:&str,inst:Arc<PackageInstance>,writer:&mut W) {
   let mut sym_gen = SymbolGenerator::new(inst.clone());
   let main_sym_name = SymbolName::parse(main_name);
   sym_gen.run(&main_sym_name,writer);
   
   

   if let Some((decl,_)) = inst.ast_pkg.find_decl(&main_sym_name) {
      match &decl.content {
         ExternalDeclarationData::FunctionDefinition(fd) => {
            if let TypeSpecifierNonArrayData::TypeName(old_ty_name) = &fd.prototype.ty.ty.ty.content {
               writer.write_fmt(format_args!("\r\nlayout(location = 0) out {} _output;\r\n",old_ty_name.0)).unwrap();
            }
            let new_func = re_generator_function(fd);
            writer.write_str("\r\n").unwrap();
            show_function_definition(writer, &new_func, &mut sym_gen.fs).unwrap();
         },
         _ => {
            log::error!("{:?} must be function",main_name);
         }
      }   
   }
}

fn re_generator_function(old_decl:&FunctionDefinition) -> FunctionDefinition {
   let fn_id = Identifier::new(IdentifierData("main".into()), None);
   let full_ty =  FullySpecifiedTypeData {
      qualifier:None,
      ty:TypeSpecifierData {
         ty:TypeSpecifierNonArrayData::Void.into(),
         array_specifier: None,
      }.into()
   };
   let new_fn_prototype = FunctionPrototypeData {
      ty:full_ty.into(),
      name:fn_id,
      parameters:vec![]
   };
   
   let mut re_gen = ReGenStmt::default();
   let new_stmt = re_gen.rum_compound_stmt(&old_decl.statement);

   FunctionDefinitionData {
      prototype:new_fn_prototype.into(),
      statement:new_stmt
   }.into()
}




#[derive(Default)]
pub struct ReGenStmt {
   lsts:Vec<Vec<Statement>>
}

impl ReGenStmt {
   pub fn run_stmt(&mut self,stmt:&Statement) -> Statement {
      match &stmt.content {
         StatementData::Compound(comp) => {
            StatementData::Compound(self.rum_compound_stmt(comp)).into()
         },
         StatementData::Jump(jump) => {
            StatementData::Jump(self.run_jump_stmt(jump)).into()
         },
         StatementData::Selection(select) => {
            StatementData::Selection(self.run_select_stmt(select)).into()
         },
         StatementData::Iteration(iter) => {
            match &iter.content {
               IterationStatementData::DoWhile(stmt,e) => {
                  StatementData::Iteration(IterationStatementData::DoWhile(Box::new(self.check_run_return_stmt(&stmt)),e.clone()).into()).into()
               },
               IterationStatementData::While(cond,stmt) => {
                  StatementData::Iteration(IterationStatementData::While(cond.clone(),Box::new(self.check_run_return_stmt(&stmt))).into()).into()
               },
               IterationStatementData::For(a,b,stmt) => {
                  StatementData::Iteration(IterationStatementData::For(a.clone(),b.clone(),Box::new(self.check_run_return_stmt(&stmt))).into()).into()
               }
            }
         },
         _ => stmt.clone()
      }
   }

   fn run_select_stmt(&mut self,select:&SelectionStatement) -> SelectionStatement {
        let rest = self.run_select_reset_stmt(&select.rest);
        SelectionStatementData {
         cond:select.cond.clone(),
         rest
        }.into()
   }

   fn run_select_reset_stmt(&mut self,select_rest:&SelectionRestStatement) -> SelectionRestStatement {
      match &select_rest.content {
         SelectionRestStatementData::Statement(stmt) => {
            
            let new_stmt = self.check_run_return_stmt(stmt);
            SelectionRestStatementData::Statement(Box::new(new_stmt)).into()
         },
         SelectionRestStatementData::Else(s1,s2) => {  
            let new_s1 = self.check_run_return_stmt(s1);
            let new_s2 = self.check_run_return_stmt(s2);
            SelectionRestStatementData::Else(Box::new(new_s1),Box::new(new_s2)).into()
         }
      }
   }

   fn check_run_return_stmt(&mut self,stmt:&Statement) -> Statement {
      if let Some(expr) = Self::get_return_stmt_expr(stmt) {
         let assign_expr = Self::make_assign_stmt(expr);
         let lst = vec![assign_expr,StatementData::Jump(JumpStatementData::Return(None).into()).into()  ];
         StatementData::Compound(CompoundStatementData { statement_list:lst }.into()).into()
      } else {
         self.run_stmt(stmt).into()
      }
   }

   fn get_return_stmt_expr(stmt:&Statement) -> Option<Expr> {
      match &stmt.content {
         StatementData::Jump(jump) => {
            match &jump.content {
               JumpStatementData::Return(v) => {
                  v.as_ref().map(|s| *s.clone())
               },
                _ => None
            }
         },
         _ => None
      }
   }

   fn make_assign_stmt(expr:Expr) -> Statement {
      let assign_expr:Expr = ExprData::Assignment(
         Box::new(ExprData::Variable("_output".into_node()).into()),
         AssignmentOpData::Equal.into(),
         expr.into()
       ).into();
       StatementData::Expression(ExprStatementData(Some(assign_expr)).into()).into()
   }

   fn run_jump_stmt(&mut self,jump:&JumpStatement) -> JumpStatement {
      match &jump.content {
         JumpStatementData::Return(Some(expr)) => {
             let assign_expr:Expr = ExprData::Assignment(
               Box::new(ExprData::Variable("_output".into_node()).into()),
               AssignmentOpData::Equal.into(),
               expr.clone().into()
             ).into();
             let assign_stmt: Statement = StatementData::Expression(ExprStatementData(Some(assign_expr)).into()).into();
             self.push_last(assign_stmt);
             JumpStatementData::Return(None).into()
         },
         _ => jump.clone() 
      }
   }

   fn rum_compound_stmt(&mut self,compound:&CompoundStatement) -> CompoundStatement {
      self.push_ctx();
      for stmt in compound.statement_list.iter() {
         let ret_stmt = self.run_stmt(&stmt);
          self.push_last(ret_stmt);
      }
      let ret = self.collect_compound();
      self.pop_ctx();
      ret
   }

   fn collect_compound(&mut self) -> CompoundStatement {
      CompoundStatementData { statement_list:self.drain_last_lst() }.into()
   }

   fn push_ctx(&mut self) {
      self.lsts.push(vec![]);
   }

   fn pop_ctx(&mut self) {
      self.lsts.pop();
   }

   fn drain_last_lst(&mut self) -> Vec<Statement> {
      if let Some(last) = self.lsts.last_mut() {
         return last.drain(..).collect()
      }
      vec![]
   }

   fn push_last(&mut self,stmt:Statement) {
      if let Some(last_ctx) = self.lsts.last_mut() {
         last_ctx.push(stmt)
      }
   }
}