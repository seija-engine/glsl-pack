use std::{fmt::Write, sync::Arc};

use crate::{shader::Shader, compiler::sym_generator::SymbolGenerator, pkg_inst::PackageInstance, ast::SymbolName};

pub fn run_vs_dep_main_step<W:Write>(shader:&Shader,main_name:&str,inst:Arc<PackageInstance>,writer:&mut W) {
   let mut sym_gen = SymbolGenerator::new(inst);
   let main_sym_name = SymbolName::parse(main_name);
   sym_gen.run(vec![main_sym_name],writer);
   //??? glsl的vertex out只能是变量不能是结构体 fuck
}