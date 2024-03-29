mod compiler;
mod dag;
mod shader_compiler;
mod sym_generator;
mod dep_search;
mod combinadics;
mod steps;
use std::{collections::HashMap, fmt::Write};
pub use dep_search::DepSearch;
pub use compiler::{compile_shader};
use glsl_pack_rtbase::shader::Shader;
use smol_str::SmolStr;

use crate::backends::Backends;



pub trait IShaderBackend {
    type ExData;
    fn write_vs_head<W:Write>(&self,_:&mut W) {}
    fn write_fs_head<W:Write>(&self,_:&mut W) {}
    fn write_common_head<W:Write>(&self,_:&mut W) {}
    fn vertex_names(&self) -> &HashMap<SmolStr,(usize,SmolStr)>;
    fn write_uniforms<W:Write>(&self,_:&mut W,_shader:&Shader,_ex_data:&Self::ExData) {}

    fn write_backend_trait<W:Write>(&self,_write:&mut W,_shader:&Shader,_backends:&Backends,_ex_data:&Self::ExData) {}
    fn write_vs_slots<W:Write>(&self,_write:&mut W,_shader:&Shader,_ex_data:&Self::ExData) {}
    fn write_fs_slots<W:Write>(&self,_write:&mut W,_shader:&Shader,_ex_data:&Self::ExData) {}
}