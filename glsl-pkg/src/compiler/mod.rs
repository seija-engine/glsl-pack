mod compiler;
mod dag;
mod shader_compiler;
mod sym_generator;
mod dep_search;
mod compile_env;
mod combinadics;
mod steps;
use std::{collections::HashMap, fmt::Write};
pub use dep_search::DepSearch;
pub use compile_env::CompileEnv;
pub use compiler::{compile_shader};
use glsl_pack_rtbase::shader::Shader;



pub trait IShaderBackend {
    fn write_vs_head<W:Write>(&self,_:&mut W) {}
    fn write_fs_head<W:Write>(&self,_:&mut W) {}
    fn write_common_head<W:Write>(&self,_:&mut W) {}
    fn vertex_names(&self) -> &HashMap<String,(usize,String)>;
    fn write_uniforms<W:Write>(&self,_:&mut W,_shader:&Shader) {}

    fn write_backend_trait<W:Write>(&self,write:&mut W,shader:&Shader,backends:&crate::backends::Backends) {}
}