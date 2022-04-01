use std::collections::HashSet;

use crate::backends::Backends;

pub struct BuildinSymbols {
    syms:HashSet<String>,
    typs:HashSet<String>,
}

impl BuildinSymbols {
    pub fn new(backends:&Backends) -> Self {
        let mut syms:HashSet<String> = HashSet::default();
        let arr = ["gl_Color","gl_SecondaryColor","gl_Normal","gl_Vertex","gl_MultiTexCoordn","gl_FogCoord",
                              "gl_Position","gl_ClipVertex","gl_PointSize","gl_FrontColor","gl_BackColor","gl_FrontSecondaryColor",
                              "gl_BackSecondaryColor","gl_TexCoord","gl_FogFragCoord","vert_position","vert_uv0",
                              "vert_uv1","vert_normal","vert_tangent","vert_color","material","texture","clamp","dot","normalize",
                              "pow","max","reflect","transpose","inverse","distance"];
        syms.extend(arr.iter().map(|s| s.to_string()));

        let mut typs:HashSet<String> = HashSet::default();
        let arr = ["void","bool","int","float","vec2","vec3",
                              "vec4","bvec2","bvec3","bvec4","ivec2","ivec3",
                              "ivec4","mat2","mat2x2","mat3","mat3x3","mat4",
                              "mat4x4","mat2x3","mat2x4","mat3x2","mat3x4","mat4x2","mat4x3",
                              "sampler1D","sampler2D","sampler3D","samplerCube","sampler1DShadow","sampler2DShadow"];
        typs.extend(arr.iter().map(|s| s.to_string()));
        
        for (_,backend) in backends.values.iter() {
            for item in backend.fns.iter() {
                let mut new_name = item.name.clone();
                if let Some(r) = new_name.get_mut(0..1) {
                    r.make_ascii_uppercase();
                }

                if let Some(arr_name) = item.array_name.as_ref() {
                    let mut new_arr_name = arr_name.clone();
                    if let Some(r) = new_arr_name.get_mut(0..1) {
                        r.make_ascii_uppercase();
                    }
                    let fname = format!("get{}{}",new_arr_name,new_name);
                    syms.insert(fname);
                } else {
                    let fname = format!("get{}",new_name);
                    syms.insert(fname);
                }
              
            }
        }
        BuildinSymbols { syms,typs }
    }

    pub fn has_type(&self,name:&str) -> bool {
        self.typs.contains(name)
    }

    pub fn has_symbol(&self,name:&str) -> bool {
        if name.starts_with("tex_") || name.starts_with("slot_") {
            return true;
        }
        self.syms.contains(name)
    }
}