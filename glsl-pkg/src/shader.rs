use std::collections::HashMap;

use serde_json::Value;
use anyhow::{Result};

use crate::errors::ShaderLoadError;

#[derive(Debug)]
pub struct Shader {
    name:String,
    pub vertexs:HashMap<String,bool>,
    pub backend:Vec<String>,
    slots:Vec<String>,
    vs_main:String,
    fs_main:String
}

impl Shader {
    pub fn from_json(value:&Value) -> Result<Shader> {
        let name = value.get("name").and_then(Value::as_str).ok_or(ShaderLoadError::JsonError("name"))?;
        
        let mut vertexs = HashMap::default();
        let json_verts = value.get("vertex").and_then(Value::as_object).ok_or(ShaderLoadError::JsonError("vertex"))?;
        for (k,v) in json_verts {
            let v_str = v.as_str().unwrap_or_default();
            if v_str == "require" {
                vertexs.insert(k.to_string(), true);
            } else {
                vertexs.insert(k.to_string(), false);
            }
        }

        let mut backend:Vec<String> = vec![];
        if let Some(vec_arr) = value.get("backend").and_then(Value::as_array) {
            backend = vec_arr.iter().filter_map(|b| b.as_str()).map(String::from).collect();
        }

        let mut slots:Vec<String> = vec![];
        if let Some(vec_arr) = value.get("slots").and_then(Value::as_array) {
            slots = vec_arr.iter().filter_map(|b| b.as_str()).map(String::from).collect();
        }
        let vs_main = value.get("vs").and_then(Value::as_str).ok_or(ShaderLoadError::JsonError("vs"))?.to_owned();
        let fs_main = value.get("fs").and_then(Value::as_str).ok_or(ShaderLoadError::JsonError("fs"))?.to_owned();
        Ok(Shader {
            name:name.to_string(),
            vertexs,
            backend,
            slots,
            vs_main,
            fs_main
        })
    }
}