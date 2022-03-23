use std::{collections::HashMap, fmt::Write};

use glsl_pack_rtbase::shader::Shader;



pub fn run_vetex_layout_step<W:Write>(vert_names:&HashMap<String,(usize,String)>,writer:&mut W,verts:&Vec<String>) {
    //需要根据position排个序
    let mut infos:Vec<(&String,usize,&String)> = vec![];
    for s in verts.iter() {
        if let Some(name_info) = vert_names.get(s) {
            infos.push((s,name_info.0,&name_info.1));
        } else {
            log::warn!("error vertex name:{}",s);
        }
    }
    infos.sort_by(|a,b| a.1.cmp(&b.1));


    for (name,idx,typ) in infos.iter() {
        writer.write_fmt(format_args!("layout(location = {}) in {} vert_{};\r\n",idx,typ,name.to_lowercase())).unwrap();
    }
}