mod compiler;
mod shader_compiler;
mod sym_generator;
mod dep_search;
mod compile_env;
mod steps;
use std::{collections::HashMap, fmt::Write};
pub use dep_search::DepSearch;
pub use compile_env::CompileEnv;
pub use compiler::{Compiler};


pub trait IShaderBackend {
    fn write_vs_head<W:Write>(&self,_:&mut W) {}
    fn vertex_names(&self) -> &HashMap<String,(usize,String)>;
    fn write_vs_after_vertex<W:Write>(&self,_:&mut W) {}

    fn trait_fns<W:Write>(&self) -> HashMap<String,fn(&mut W)> { HashMap::default() }
}



//test
pub struct SeijaShaderBackend {
    vertexs:HashMap<String,(usize,String)>
}

impl SeijaShaderBackend {
    pub fn new() -> Self {
        let mut vertexs = HashMap::new();
        vertexs.insert("POSITION".into(), (0,"vec3".into()));
        vertexs.insert("UV0".into(), (1,"vec2".into()));
        vertexs.insert("UV1".into(), (2,"vec2".into()));
        vertexs.insert("NORMAL".into(), (3,"vec3".into()));
        vertexs.insert("TANGENT".into(), (4,"vec3".into()));
        vertexs.insert("COLOR".into(), (5,"vec4".into()));
        SeijaShaderBackend { vertexs }
    }
}


impl IShaderBackend for SeijaShaderBackend {
    fn write_vs_head<W:Write>(&self,writer:&mut W) {
        writer.write_str("#version 450\r\n").unwrap();
    }
    fn vertex_names(&self) -> &HashMap<String,(usize,String)> {
       &self.vertexs
    }

    fn trait_fns<W:Write>(&self) -> HashMap<String, fn(&mut W)> {
        let mut traits:HashMap<String,fn(&mut W)> = HashMap::default();
        traits.insert("Camera".into(), get_camera_trait);
        traits
    }

    fn write_vs_after_vertex<W:Write>(&self, writer:&mut W) {
        writer.write_str("layout(set = 0, binding = 0) uniform FrameUniforms {\r\n").unwrap();
        writer.write_str("  mat4 cameraVP;\r\n").unwrap();
        writer.write_str("  mat4 cameraView;\r\n").unwrap();
        writer.write_str("  mat4 cameraP;\r\n").unwrap();
        writer.write_str("  vec4 cameraPos;\r\n").unwrap();
        writer.write_str("} frameUniforms;\r\n").unwrap();

        writer.write_str("layout(set = 1, binding = 0) uniform ObjectUniforms {\r\n").unwrap();
        writer.write_str("  mat4 transform;\r\n").unwrap();
        writer.write_str("} objectUniforms;\r\n").unwrap();
       
    }
}

fn get_camera_trait<W:Write>(writer:&mut W) {
    writer.write_str("mat4 getCameraView() {\r\n").unwrap();
    writer.write_str("  return frameUniforms.cameraView;\r\n").unwrap();
    writer.write_str("}\r\n").unwrap();

    writer.write_str("mat4 getCameraViewProject() {\r\n").unwrap();
    writer.write_str("  return frameUniforms.cameraVP;\r\n").unwrap();
    writer.write_str("}\r\n").unwrap();
}