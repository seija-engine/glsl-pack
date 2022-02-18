use std::collections::{HashMap};

#[derive(Debug)]
pub struct Shader {
    pub name:String,
    pub vertexs:HashMap<String,bool>,
    pub backend:Vec<String>,
    pub slots:Vec<String>,
    pub vs_main:String,
    pub fs_main:String
}
