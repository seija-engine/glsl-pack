use std::collections::HashMap;
use smol_str::{SmolStr};
use glsl_pack_rtbase::shader::Shader;
use serde_json::Value;
use anyhow::{Result};

use crate::errors::ShaderLoadError;


pub fn read_shader_from_json(value:&Value) -> Result<Shader> {
    let name = value.get("name").and_then(Value::as_str).ok_or(ShaderLoadError::JsonError("name"))?;
    
    let mut vertexs = HashMap::default();
    let json_verts = value.get("vertex").and_then(Value::as_object).ok_or(ShaderLoadError::JsonError("vertex"))?;
    for (k,v) in json_verts {
        let v_str = v.as_str().unwrap_or_default();
        if v_str == "require" {
            vertexs.insert(k.into(), true);
        } else {
            vertexs.insert(k.into(), false);
        }
    }

    let mut backend:Vec<SmolStr> = vec![];
    if let Some(vec_arr) = value.get("backend").and_then(Value::as_array) {
        backend = vec_arr.iter().filter_map(|b| b.as_str()).map(SmolStr::new).collect();
    }

    let mut slots:Vec<String> = vec![];
    if let Some(vec_arr) = value.get("slots").and_then(Value::as_array) {
        slots = vec_arr.iter().filter_map(|b| b.as_str()).map(String::from).collect();
    }
    let vs_main = value.get("vs").and_then(Value::as_str).ok_or(ShaderLoadError::JsonError("vs"))?.to_owned();
    let fs_main = value.get("fs").and_then(Value::as_str).ok_or(ShaderLoadError::JsonError("fs"))?.to_owned();
    Ok(Shader {
        name:name.into(),
        vertexs,
        backend,
        features:HashMap::default(),
        slots,
        vs_main,
        fs_main
    })
}