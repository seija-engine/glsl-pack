use std::{collections::HashMap, fmt::Write};
use smol_str::SmolStr;


pub fn run_vetex_layout_step<W:Write>(vert_names:&HashMap<SmolStr,(usize,SmolStr)>,writer:&mut W,verts:&Vec<SmolStr>) {
    //需要根据position排个序
    let mut infos:Vec<(&SmolStr,usize,&SmolStr)> = vec![];
    for s in verts.iter() {
        if let Some(name_info) = vert_names.get(s.as_str()) {
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