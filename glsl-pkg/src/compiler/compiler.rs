use std::{path::{PathBuf, Path}, fs, sync::Arc};

use crate::{MacroGroup, CompileEnv, compiler::shader_compiler::ShaderCompiler, shader::Shader};

use super::{IShaderBackend, combinadics::start_combination};







pub struct CompileConfig<T:IShaderBackend> {
    source_path:PathBuf,
    out_path:PathBuf,
    macro_group:MacroGroup,
    backend:T
}

impl<T> CompileConfig<T> where T:IShaderBackend {
    pub fn new(backend:T) -> Self {
        CompileConfig { backend,source_path:Default::default(),out_path:Default::default(),macro_group:MacroGroup::default()}
    }
    pub fn set_source_path<P:AsRef<Path>>(&mut self,path:P) {
        self.source_path = path.as_ref().into();
    }

    pub fn set_out_path<P:AsRef<Path>>(&mut self,path:P) {
        self.out_path = path.as_ref().into();
    }

    pub fn set_macros(&mut self,group:MacroGroup) {
        self.macro_group = group
    }
}


pub struct Compiler<T:IShaderBackend> {
    config:CompileConfig<T>,
    env:CompileEnv
}

impl<T> Compiler<T> where T:IShaderBackend {
    pub fn new(config:CompileConfig<T>) -> Self {
        Compiler { config,env:CompileEnv::new() }
    }

    pub fn run_task(&mut self,task:&CompileTask) {
        if !self.config.out_path.exists() {
            fs::create_dir_all(&self.config.out_path).unwrap();
        }
        let new_macro_group = self.config.macro_group.join_to(task.macros.clone());     
        let pkg_inst = self.env.get_pkg_inst(&self.config.source_path, &new_macro_group);
        let find_shader = pkg_inst.info.shaders.iter().find(|v| v.name == task.shader_name);
        if let Some(shader) = find_shader {
            let mut old_macros = new_macro_group.names.clone();
            let mut opt_names:Vec<String> = vec![];
            for (name,is_require) in shader.vertexs.iter() {
                if *is_require {
                    old_macros.push(name.to_string());
                } else {
                    opt_names.push(name.clone());
                }
            }
            
            let mut requires:Vec<String> = vec![];
            let mut options:Vec<String> = vec![];
            for (v,is_require) in shader.vertexs.iter() {
                let mut nv = "HAS_".to_string();
                nv.push_str(v.as_str());
                if *is_require {
                    requires.push(nv);
                } else {
                    options.push(nv);
                }
            }

            start_combination(options.len(), |idxs| {
                let mut all_macros:Vec<String> = vec![];
                for (idx,is_use) in idxs.iter() {
                    if *is_use {
                        all_macros.push(options[*idx].clone());
                    }
                }
                for v in requires.iter() {
                    all_macros.push(v.to_string());
                }
                let macro_group = MacroGroup::new(vec![new_macro_group.names.clone(),all_macros].concat());
                self.run_macro(shader, &macro_group);
            });
            

        } else {
            log::error!("not found shader:{}",task.shader_name);
        }
    }


    fn run_macro(&mut self,shader:&Arc<Shader>,macros:&MacroGroup) {
        let macro_hash = macros.hash_base64();
        let pkg_inst = self.env.get_pkg_inst(&self.config.source_path, &macros);
        let mut vs_string = String::default();
        let mut fs_string:String = String::default();
        let mut shader_compiler = ShaderCompiler::new(shader.clone(),pkg_inst.clone());
        shader_compiler.compile(&self.config.backend,&mut vs_string,&mut fs_string);
        let vs_file_name = format!("{}_{}.vert",shader.name,macro_hash); 
        let fs_file_name = format!("{}_{}.frag",shader.name,macro_hash);
        std::fs::write(self.config.out_path.join(vs_file_name), vs_string).unwrap();
        std::fs::write(self.config.out_path.join(fs_file_name), fs_string).unwrap();
        
    }

   
}

pub struct  CompileTask {
    macros:Vec<String>,
    shader_name:String
}

impl CompileTask {
    pub fn new(name:&str,macros:Vec<String>) -> Self {
        CompileTask { macros: macros, shader_name: name.to_owned() }
    }
}




#[test]
fn test_compiler() {
    use super::SeijaShaderBackend;
    env_logger::init();
    let mut config = CompileConfig::new(SeijaShaderBackend::new());
    config.set_macros(MacroGroup::new(vec!["GLOBAL_M".to_string()]));
    config.set_source_path("../tests/core/");
    config.set_out_path("../tests/output/");
    let mut compiler = Compiler::new(config);
    
    compiler.run_task(&CompileTask::new("color", vec!["MATERIAL_M".to_owned()]));
}