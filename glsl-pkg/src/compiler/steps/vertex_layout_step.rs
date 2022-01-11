use std::{collections::HashMap, fmt::Write};

use crate::shader::Shader;


pub fn run_vetex_layout_step<W:Write>(shader:&Shader,vert_names:&HashMap<String,(usize,String)>,writer:&mut W) {
    //需要根据position排个序
    let mut infos:Vec<(&String,usize,&String,bool)> = vec![];
    for (k,v) in shader.vertexs.iter() {
        if let Some(name_info) = vert_names.get(k) {
            infos.push((k,name_info.0,&name_info.1,*v));
        } else {
            log::warn!("error vertex name:{}",k);
        }
    }
    infos.sort_by(|a,b| a.1.cmp(&b.1));


    for (name,idx,typ,is_require) in infos.iter() {
        if *is_require {
            writer.write_fmt(format_args!("layout(location = {}) {} vert_{};\r\n",idx,typ,name.to_lowercase())).unwrap();
        } else {
            writer.write_fmt(format_args!("#ifdef VERTEX_{}\r\n",name.to_uppercase())).unwrap();
            writer.write_fmt(format_args!("layout(location = {}) {} vert_{};\r\n",idx,typ,name.to_lowercase())).unwrap();
            writer.write_str("#endif\r\n").unwrap();
        }
    }
}