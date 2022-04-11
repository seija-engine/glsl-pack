use std::{sync::Arc};
use text_macro::{MacroContext};
use crate::{MacroGroup, package::{PackageInfo}, walk_glsl_folder, ast::{ASTPackage}};

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
            let code_string = f.to_string();
            if let Err(err) = inst.ast_pkg.load_file(inst.info.name.clone(),&r_path.replace('\\', "/").trim_start_matches('/'), code_string) {
                println!("parse file error{:}",&r_path);
                dbg!(err);
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