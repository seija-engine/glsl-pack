use std::{fmt::Write, collections::HashMap};

use glsl_pack_rtbase::shader::Shader;



pub fn run_shader_trait_step<W:Write>(shader:&Shader,trait_fns:&HashMap<String,fn(&mut W)>,writer:&mut W) {
    for trait_name in shader.backend.iter() {
        if let Some(trait_fn) = trait_fns.get(trait_name.as_str()) {
            trait_fn(writer);
        } else {
            log::warn!("shader trait not found:{}",trait_name);
        }
    }
}