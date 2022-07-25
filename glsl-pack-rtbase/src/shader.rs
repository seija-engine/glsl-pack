use std::collections::{HashMap};
use smol_str::{SmolStr};
#[derive(Debug)]
pub struct Shader {
    pub name:SmolStr,
    pub vertexs:HashMap<SmolStr,bool>,
    pub backend:Vec<SmolStr>,
    pub features:HashMap<SmolStr,Vec<SmolStr>>,
    pub slots:Vec<String>,
    pub vs_main:String,
    pub fs_main:String
}
