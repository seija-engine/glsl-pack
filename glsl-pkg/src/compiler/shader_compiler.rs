use std::{sync::Arc, fmt::Write};

use glsl_pack_rtbase::shader::Shader;
use smol_str::SmolStr;

use crate::{ pkg_inst::PackageInstance, ast::{SymbolName}, BACKENDS};

use super::{steps::*, IShaderBackend};

pub struct ShaderCompiler {
    shader:Arc<Shader>,
    pkg_inst:Arc<PackageInstance>
}

impl ShaderCompiler {
    pub fn new(shader:Arc<Shader>,pkg_inst:Arc<PackageInstance>) -> Self {
        ShaderCompiler { shader,pkg_inst }
    }

    pub fn compile<B:IShaderBackend,W:Write>(&mut self,backend:&B,vs:&mut W,fs:&mut W,verts:&Vec<SmolStr>,ex_data:&B::ExData) {
       let ret_type = self.compile_vs(backend,vs,verts,ex_data);
       self.compile_fs(backend, fs,ret_type,ex_data);
    }

    fn compile_vs<B:IShaderBackend,W:Write>(&mut self,backend:&B,writer:&mut W,verts:&Vec<SmolStr>,ex_data:&B::ExData) -> Option<SymbolName>  {
        backend.write_common_head(writer);
        backend.write_vs_head(writer);
        run_vetex_layout_step(&backend.vertex_names(),writer,verts);
        backend.write_uniforms(writer,&self.shader,ex_data);
        backend.write_backend_trait(writer, &self.shader,&BACKENDS,ex_data);
       
        run_vs_dep_main_step(&self.shader, &self.shader.vs_main,self.pkg_inst.clone(), writer,|writer: &mut W| {
            backend.write_vs_slots(writer, &self.shader, ex_data);
        })

    }

    fn compile_fs<B:IShaderBackend,W:Write>(&mut self,backend:&B,writer:&mut W,in_type:Option<SymbolName>,ex_data:&B::ExData) {
        backend.write_common_head(writer);
        backend.write_fs_head(writer);
        backend.write_uniforms(writer,&self.shader,ex_data);
        backend.write_backend_trait(writer, &self.shader,&BACKENDS,ex_data);
        run_fs_dep_main_step(&self.shader.fs_main,self.pkg_inst.clone(),writer,in_type,|wirter| {
            backend.write_fs_slots(wirter, &self.shader, ex_data);
        });
    }
}

