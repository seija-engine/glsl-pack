use std::{collections::{HashMap}, convert::TryFrom};
use serde_json::Value;
use smol_str::{SmolStr};

use crate::rt_shaders::{get_features_macro_list, get_features_backend_list};
#[derive(Debug)]
pub struct Shader {
    pub name:SmolStr,
    pub vertexs:HashMap<SmolStr,bool>,
    pub backend:Vec<SmolStr>,
    pub features:HashMap<SmolStr,FeatureItem>,
    pub slots:Vec<String>,
    pub vs_main:String,
    pub fs_main:String
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct FeatureItem {
   pub macros:Vec<SmolStr>,
   pub backends:Vec<SmolStr>
}

impl Shader {
    pub fn get_macros(&self,names:&Vec<SmolStr>) -> Vec<SmolStr> {  get_features_macro_list(&self.features,names) }

    pub fn get_backends(&self,names:&Vec<SmolStr>) -> Vec<SmolStr> { 
         let mut lst = get_features_backend_list(&self.features,names);
         lst.extend(self.backend.clone());
         lst
    }
}


impl TryFrom<&Value> for FeatureItem {
    type Error = ();
    fn try_from(value: &Value) -> Result<Self, ()> {
        let map = value.as_object().ok_or(())?;
        let mut macros:Vec<SmolStr> = vec![];
        let mut backends:Vec<SmolStr> = vec![];

        if let Some(macros_list) = map.get("macros").and_then(Value::as_array) {
            for json_macro in macros_list.iter() {
                if let Some(str_macro) = json_macro.as_str() {
                    macros.push(str_macro.into());
                }
            }
        }
        if let Some(backends_list) = map.get("backends").and_then(Value::as_array) {
            for json_backend in backends_list.iter() {
                if let Some(str_backend) = json_backend.as_str() {
                    backends.push(str_backend.into());
                }
            }
        }
        Ok(FeatureItem {
            macros,
            backends
        })
    }
}