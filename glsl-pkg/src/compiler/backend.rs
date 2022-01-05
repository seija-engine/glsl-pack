use crate::shader::Shader;

pub trait IShaderBackend {
    fn out_back_string(info:&Shader) -> String;
}

pub struct VertexBackend;

impl IShaderBackend for VertexBackend {
    fn out_back_string(info:&Shader) -> String {
        let mut out_string = String::default();
        for (name,is_require) in info.vertexs.iter() {
            
        }
        String::default()
    }
}