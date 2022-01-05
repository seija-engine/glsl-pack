use std::sync::Arc;

use crate::{shader::Shader, pkg_inst::PackageInstance};

pub struct ShaderCompiler {
    shader:Arc<Shader>,
    pkg_inst:Arc<PackageInstance>
}

impl ShaderCompiler {
    pub fn new(shader:Arc<Shader>,pkg_inst:Arc<PackageInstance>) -> Self {
        ShaderCompiler { shader,pkg_inst }
    }

    pub fn compile(&mut self) {
       
    }

    fn compile_vs(&mut self,pkg_inst:&PackageInstance) {
        
    }
}