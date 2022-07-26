use std::{collections::HashMap, path::Path};
use crate::shader::{Shader, FeatureItem};
use smol_str::{SmolStr};

#[derive(Serialize,Deserialize,Debug)]
pub struct RTShaderInfo {
   pub backends:Vec<SmolStr>,
   pub verts:HashMap<SmolStr,bool>,
   pub features:HashMap<SmolStr,FeatureItem>,
}

impl From<&Shader> for RTShaderInfo {
    fn from(shader: &Shader) -> Self {
        
        RTShaderInfo { 
            backends:shader.backend.clone(),
            verts:shader.vertexs.clone(),
            features:shader.features.clone()
        }
    }
}



#[derive(Default,Serialize,Deserialize)]
pub struct RuntimeShaders {
    pub shaders:HashMap<String,RTShaderInfo>
}

impl RuntimeShaders {
    pub fn add_shader(&mut self,pkg_name:&str,shader:&Shader) {
        let mut key = String::from(pkg_name);
        key.push('.');
        key.push_str(&shader.name);
        self.shaders.insert(key, shader.into());
    }

    pub fn write_to(&self,path:&Path) {
        let serialized = serde_json::to_string(self).unwrap();
        std::fs::write(path, serialized).unwrap();
    }

    pub fn read_from(path:&Path) -> Option<RuntimeShaders> {
        let json_string = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&json_string).ok()
    }
}