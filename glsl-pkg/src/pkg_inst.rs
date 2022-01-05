use std::{sync::Arc};
use text_macro::{MacroContext};
use crate::{MacroGroup, package::{PackageInfo, Package}, walk_glsl_folder, ast::{ASTPackage, SymbolName}, compiler::CompileEnv, DepSearchGen};

#[derive(Debug)]
pub struct PackageInstance {
    pub info:Arc<PackageInfo>,
    pub ast_pkg:ASTPackage
}

impl PackageInstance {
    pub fn create(group:MacroGroup,info:Arc<PackageInfo>) -> PackageInstance { 
        let mut inst = PackageInstance { info,ast_pkg:ASTPackage::default() };
        inst.ast_pkg.pkg_info = inst.info.clone();

        let mc = inst.load_macro_ctx(&group);
        let full_path = inst.info.path.canonicalize().unwrap().to_str().unwrap().to_string();
        for (name,f) in mc.files {
            let p_str = name.canonicalize().unwrap().to_str().unwrap().to_string();
            let r_path = p_str.trim_start_matches(&full_path);
            if let Err(err) = inst.ast_pkg.load_file(inst.info.name.clone(),&r_path.replace('\\', "/").trim_start_matches('/'), f.to_string()) {
               log::error!("{:?}",err);
            }
        }
        inst
    }

    fn load_macro_ctx(&mut self,group:&MacroGroup) -> MacroContext {
        let mut mc = MacroContext::new();
        for name in group.names.iter() {
            mc.add_define(name)
        }
        for f in walk_glsl_folder(&self.info.path) {
            mc.load_file(f.path());
        }
        mc.exp_all();
        mc
    }
}

#[test]
fn load_package() {
    
    env_logger::init();
    let mut pkg = Package::load("../tests/core/").unwrap();
    let macros = &MacroGroup::new(vec!["HAS_POSITION".to_string()]);
    let sym_vs = SymbolName::parse("core.color.vs_main");
    let sym_fs = SymbolName::parse("core.color.fs_main");

    let mut env = CompileEnv::new();
    let inst = pkg.get_inst(macros);
    let mut search_gen = DepSearchGen::new(inst);
    let mut out_string = String::default();
    search_gen.run(vec![sym_vs],&mut out_string,&mut env);

    println!("{}",out_string);
}