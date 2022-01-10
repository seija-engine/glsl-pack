use std::sync::Arc;

use crate::{shader::Shader, pkg_inst::PackageInstance};

use super::steps::run_vetex_layout_step;

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

    fn compile_vs(&mut self) {
        run_vetex_layout_step(&self.shader); // step 1  输出顶点layout
        
        // step 2  输出Backend的layout和函数
        // step 3  搜索输出vs_main  //?? 需要替换返回值为void，然后根据返回值生成VSOutput
        
    
    }
}