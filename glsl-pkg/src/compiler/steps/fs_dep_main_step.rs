use std::{fmt::Write, sync::Arc};

use glsl_lang::{ast::*, transpiler::glsl::show_function_definition};

use crate::{pkg_inst::PackageInstance, compiler::sym_generator::SymbolGenerator, ast::SymbolName};

pub fn run_fs_dep_main_step<W:Write>(main_name:&str,inst:Arc<PackageInstance>,writer:&mut W,sym:Option<SymbolName>) {
    let mut sym_gen = SymbolGenerator::new(inst.clone());
    let main_sym_name = SymbolName::parse(main_name);
    sym_gen.run(&main_sym_name,writer,false);

    if let Some(in_sym) = sym {
        writer.write_str("\r\n").unwrap();
        writer.write_fmt(format_args!("layout(location = 0) in {} _input;\r\n",&in_sym.name)).unwrap();
    }

    if let Some((decl,file)) = inst.ast_pkg.find_decl(&main_sym_name) {
        match &decl.content {
            ExternalDeclarationData::FunctionDefinition(fd) => {
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

fn find_fst_param_name(fd:&FunctionDefinition) -> String {
    for param in fd.prototype.parameters.iter() {
        match &param.content {
            FunctionParameterDeclarationData::Named(_,named) => {
               return named.ident.ident.0.to_string();
            },
            _ => {}
        }
    }
    "_input".to_string()
}

fn re_generator_function(old_decl:&FunctionDefinition) -> FunctionDefinition {
    let param_name = find_fst_param_name(old_decl);
    todo!()
}

#[derive(Default)]
pub struct ReGenStmt {
   lsts:Vec<Vec<Statement>>
}
