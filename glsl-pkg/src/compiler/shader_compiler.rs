use std::{sync::Arc, fmt::Write};

use crate::{shader::Shader, pkg_inst::PackageInstance};

use super::{steps::run_vetex_layout_step, IShaderBackend, compiler::{CompileConfig, Compiler}, CompileEnv};

pub struct ShaderCompiler {
    shader:Arc<Shader>,
    pkg_inst:Arc<PackageInstance>
}

impl ShaderCompiler {
    pub fn new(shader:Arc<Shader>,pkg_inst:Arc<PackageInstance>) -> Self {
        ShaderCompiler { shader,pkg_inst }
    }

    pub fn compile<B:IShaderBackend,W:Write>(&mut self,backend:&B,write:&mut W) {
       self.compile_vs(backend,write);
    }

    fn compile_vs<B:IShaderBackend,W:Write>(&mut self,backend:&B,writer:&mut W) {
        run_vetex_layout_step(&self.shader,&backend.vertex_names(),writer);
        
        // step 2  输出Backend的layout和函数
        // step 3  搜索输出vs_main  //?? 需要替换返回值为void，然后根据返回值生成VSOutput
    }
}


#[test]
fn test_compiler() {
    let mut config = CompileConfig::default();
    config.set_path("../tests/core/");
    let mut compiler = Compiler::new(config);
    let mut env = CompileEnv::default();
    compiler.run(&mut env);
}