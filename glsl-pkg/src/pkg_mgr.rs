use std::{path::{PathBuf}, str::FromStr, collections::HashMap, fs};
use crate::{compiler::compile_shader, IShaderBackend, backends::Backends};
use glsl_pack_rtbase::{MacroGroup, rt_shaders::RuntimeShaders};

use crate::{package::Package, compiler};



pub struct PackageManager {
    pkgs:HashMap<String,Package>,
    folders:Vec<PathBuf>,
    out_path:PathBuf,
    rt_shaders:RuntimeShaders
}

impl PackageManager {
    pub fn new() -> Self {
        PackageManager { 
            pkgs:HashMap::default(), 
            folders:vec![],
            out_path:PathBuf::from_str("./.shader_out").unwrap(),
            rt_shaders:RuntimeShaders::default()
        }
    }

    pub fn add_dir(&mut self,path:&str) {
         let path_buf = PathBuf::from_str(path).unwrap();
         self.folders.push(path_buf);
    }

    pub fn set_out_path(&mut self,path:&str) {
        let path_buf = PathBuf::from_str(path).unwrap();
        self.out_path = path_buf;
    }

    pub fn compile<B:IShaderBackend>(&mut self,pkg_name:&str,shader_name:&str,macros:&Vec<String>,backend:&B,ex_data:&B::ExData) -> bool {
        if !self.out_path.exists() {
            fs::create_dir_all(&self.out_path).unwrap();
        }
        let out_path = self.out_path.clone();
        if let Some(package) = self.get_or_load_pkg(pkg_name) {
         if let Some(shader) = compile_shader(package,shader_name,macros,out_path,backend,ex_data) {
            self.rt_shaders.add_shader(pkg_name, &shader);
            true
         } else {
             false
         }
        } else {
            log::error!("not found package:{}",pkg_name);
            false   
        }
    }
    pub fn write_rtinfos(&self) {
        let path = self.out_path.join("rt.json");
        self.rt_shaders.write_to(&path);
    } 

    fn get_or_load_pkg(&mut self,pkg_name:&str) -> Option<&mut Package> {
        if self.pkgs.contains_key(pkg_name) {
            return self.pkgs.get_mut(pkg_name)            
        }
        for path in self.folders.iter() {
            let json_path = path.join(pkg_name).join("package.json");
            if json_path.exists() {
               let pkg_path = path.join(pkg_name);
               match Package::load(pkg_path)  {
                 Ok(v) => {
                    self.pkgs.insert(pkg_name.to_string(), v);
                    return self.pkgs.get_mut(pkg_name);
                 },
                 Err(err) => {
                     log::error!("load package error path:{} err:{:?}",pkg_name,err);
                 }
               }
            }
        }
        None
    }
}

