use std::{path::{PathBuf, Path}, sync::Arc};

use crate::{MacroGroup, shader::Shader, pkg_inst::PackageInstance};

use super::{compile_env::CompileEnv, shader_compiler::ShaderCompiler, SeijaShaderBackend};


#[derive(Default)]
pub struct CompileConfig {
    path:PathBuf,
    macro_group:MacroGroup
}

impl CompileConfig {
    pub fn set_path<P:AsRef<Path>>(&mut self,path:P) {
        self.path = path.as_ref().into();
    }

    pub fn set_macros(&mut self,group:MacroGroup) {
        self.macro_group = group
    }
}


pub struct Compiler {
    config:CompileConfig
}

impl Compiler {
    pub fn new(config:CompileConfig) -> Self {
        Compiler { config }
    }

    pub fn run(&mut self,cache:&mut CompileEnv) {
        let mut vs_string = String::default();
        let mut fs_string = String::default();

        let backend = SeijaShaderBackend::new();
        let pkg_inst = cache.get_pkg_inst(&self.config.path, &self.config.macro_group);
        for shader in pkg_inst.info.shaders.iter() {
            let mut shader_compiler = ShaderCompiler::new(shader.clone(),pkg_inst.clone());
            shader_compiler.compile(&backend,&mut vs_string,&mut fs_string);
        }

        std::fs::write(pkg_inst.info.path.join("../testOut.vert"), vs_string);
        std::fs::write(pkg_inst.info.path.join("../testOut.frag"), fs_string);
    }
}

