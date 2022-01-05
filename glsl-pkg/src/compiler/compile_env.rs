use std::{path::{Path, PathBuf}, collections::{HashMap, HashSet}, sync::Arc};
use crate::{MacroGroup, package::Package, pkg_inst::PackageInstance};

#[derive(Default)]
pub struct CompileEnv {
    packages:HashMap<PathBuf,Package>,
    buildin_typeset:HashSet<String>,
    buildin_fnset:HashSet<String>
}

impl CompileEnv {

    pub fn new() -> Self {
        let buildin_typeset:HashSet<String> = vec!["float","int","void"].into_iter().map(String::from).collect();
        let buildin_fnset:HashSet<String> = vec!["max"].into_iter().map(String::from).collect();
        CompileEnv { packages:HashMap::default(),buildin_typeset,buildin_fnset }
    }

    pub fn get_pkg_inst<P:AsRef<Path>>(&mut self,path:P,macros:&MacroGroup) -> Arc<PackageInstance> {
        let full_path = path.as_ref().canonicalize().unwrap();
        if !self.packages.contains_key(&full_path) {
            let new_pkg = Package::load(path).unwrap();
            self.packages.insert(full_path.clone(), new_pkg);
        }
        return self.packages.get_mut(&full_path).unwrap().get_inst(macros)
    }

    pub fn is_buildin_type(&self,name:&str) -> bool {
        self.buildin_typeset.contains(name)
    }

    pub fn is_buildin_fn(&self,name:&str) -> bool {
        self.buildin_fnset.contains(name)
    }

    pub fn is_buildin_sym(&self,name:&str) -> bool {
        self.is_buildin_type(name) || self.is_buildin_fn(name)
    }

    
}