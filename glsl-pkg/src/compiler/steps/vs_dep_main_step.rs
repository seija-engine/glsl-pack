use std::{fmt::Write, sync::Arc};

use glsl_lang::{ast::*, transpiler::glsl::show_function_definition};
use glsl_pack_rtbase::shader::Shader;

use crate::{compiler::sym_generator::SymbolGenerator, pkg_inst::PackageInstance, ast::SymbolName};

use super::ReplaceReturnStmt;

pub fn run_vs_dep_main_step<F:FnOnce(&mut W),W:Write>(_shader:&Shader,main_name:&str,inst:Arc<PackageInstance>,writer:&mut W,f:F) -> Option<SymbolName> {
   let mut sym_gen = SymbolGenerator::new(inst.clone());
   let main_sym_name = SymbolName::parse(main_name);
   sym_gen.run(&main_sym_name,writer,false);
   
   let mut find_ret_type:Option<SymbolName> = None;
   if let Some((decl,file)) = inst.ast_pkg.find_decl(&main_sym_name) {
      match &decl.content {
         ExternalDeclarationData::FunctionDefinition(fd) => {
            if let TypeSpecifierNonArrayData::TypeName(old_ty_name) = &fd.prototype.ty.ty.ty.content {
               find_ret_type = file.find_sym(&old_ty_name.0, &inst.ast_pkg);
               writer.write_fmt(format_args!("\r\nlayout(location = 0) out {} _output;\r\n",old_ty_name.0)).unwrap();
            }
            f(writer);
            let new_func = re_generator_function(fd);
            writer.write_str("\r\n").unwrap();
            show_function_definition(writer, &new_func, &mut sym_gen.fs).unwrap();
         },
         _ => {
            log::error!("{:?} must be function",main_name);
         }
      }   
   } else {
      log::error!("not found main symbol:{:?}",&main_sym_name);
   }
   find_ret_type
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
   
   let mut re_gen = ReplaceReturnStmt::default();
   let new_stmt = re_gen.rum_compound_stmt(&old_decl.statement);

   FunctionDefinitionData {
      prototype:new_fn_prototype.into(),
      statement:new_stmt
   }.into()
}