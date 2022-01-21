mod vertex_layout_step;
mod shader_trait_step;
mod vs_dep_main_step;
mod fs_dep_main_step;
use glsl_lang::ast::*;
pub use vertex_layout_step::{run_vetex_layout_step};
pub use shader_trait_step::{run_shader_trait_step};
pub use vs_dep_main_step::{run_vs_dep_main_step};
pub use fs_dep_main_step::{run_fs_dep_main_step};

pub struct ReplaceReturnStmt {
    lsts:Vec<Vec<Statement>>,
    replace_name:String,
}

impl Default for ReplaceReturnStmt {
    fn default() -> Self {
        ReplaceReturnStmt { lsts: vec![], replace_name: String::from("_output") }
    }
}

impl ReplaceReturnStmt {
    pub fn replace_name(&mut self,name:&str) {
        self.replace_name = name.to_string();
    }

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
          let assign_expr = Self::make_assign_stmt(expr,self.replace_name.as_str());
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
 
    fn make_assign_stmt(expr:Expr,name:&str) -> Statement {
       let s:IdentifierData = name.into();
       let assign_expr:Expr = ExprData::Assignment(
          Box::new(ExprData::Variable(s.into()).into()),
          AssignmentOpData::Equal.into(),
          expr.into()
        ).into();
        StatementData::Expression(ExprStatementData(Some(assign_expr)).into()).into()
    }
 
    fn run_jump_stmt(&mut self,jump:&JumpStatement) -> JumpStatement {
       match &jump.content {
          JumpStatementData::Return(Some(expr)) => {
            let s: IdentifierData = self.replace_name.as_str().into();
              let assign_expr:Expr = ExprData::Assignment(
                Box::new(ExprData::Variable(s.into()).into()),
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