use std::{sync::Arc, fmt::Write};

use crate::{shader::Shader, pkg_inst::PackageInstance, ast::{RcSymbolName, SymbolName}};

use super::{steps::*, IShaderBackend};

pub struct ShaderCompiler {
    shader:Arc<Shader>,
    pkg_inst:Arc<PackageInstance>
}

impl ShaderCompiler {
    pub fn new(shader:Arc<Shader>,pkg_inst:Arc<PackageInstance>) -> Self {
        ShaderCompiler { shader,pkg_inst }
    }

    pub fn compile<B:IShaderBackend,W:Write>(&mut self,backend:&B,vs:&mut W,fs:&mut W) {
       let ret_type = self.compile_vs(backend,vs);
       self.compile_fs(backend, fs,ret_type);
    }

    fn compile_vs<B:IShaderBackend,W:Write>(&mut self,backend:&B,writer:&mut W) -> Option<SymbolName>  {
        backend.write_common_head(writer);
        backend.write_vs_head(writer);
        run_vetex_layout_step(&self.shader,&backend.vertex_names(),writer);
        backend.write_uniforms(writer);
        run_shader_trait_step(&self.shader, &backend.trait_fns(), writer);
        run_vs_dep_main_step(&self.shader, &self.shader.vs_main,self.pkg_inst.clone(), writer)
    }

    fn compile_fs<B:IShaderBackend,W:Write>(&mut self,backend:&B,writer:&mut W,in_type:Option<SymbolName>) {
        backend.write_common_head(writer);
        backend.write_fs_head(writer);
        backend.write_uniforms(writer);
        run_shader_trait_step(&self.shader, &backend.trait_fns(), writer);
        run_fs_dep_main_step(&self.shader.fs_main,self.pkg_inst.clone(),writer,in_type);
    }
}

