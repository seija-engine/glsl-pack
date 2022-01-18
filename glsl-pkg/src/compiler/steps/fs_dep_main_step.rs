use std::{fmt::Write, sync::Arc};

use crate::{pkg_inst::PackageInstance, compiler::sym_generator::SymbolGenerator, ast::SymbolName};

pub fn run_fs_dep_main_step<W:Write>(main_name:&str,inst:Arc<PackageInstance>,writer:&mut W,sym:Option<SymbolName>) {
    let mut sym_gen = SymbolGenerator::new(inst.clone());
    let main_sym_name = SymbolName::parse(main_name);
    sym_gen.run(&main_sym_name,writer,false);

    if let Some(in_sym) = sym {
        writer.write_str("\r\n").unwrap();
        writer.write_fmt(format_args!("layout(location = 0) in {} _input;\r\n",&in_sym.name)).unwrap();
    }
}