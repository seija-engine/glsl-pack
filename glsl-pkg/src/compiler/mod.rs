mod compiler;
mod shader_compiler;
mod sym_generator;
mod dep_search;
mod compile_env;
mod backend;
mod steps;


use std::collections::HashMap;

pub use dep_search::DepSearch;
pub use compile_env::CompileEnv;

pub trait IShaderBackend {
    fn vertex_names(&self) -> &HashMap<String,(usize,String)>;
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
    fn vertex_names(&self) -> &HashMap<String,(usize,String)> {
       &self.vertexs
    }
}