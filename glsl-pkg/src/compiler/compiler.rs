use std::{path::{PathBuf}, sync::{Arc}, collections::{hash_map::DefaultHasher, HashSet}, hash::{Hash, Hasher}};

use glsl_pack_rtbase::shader::Shader;
use shaderc::Compiler;
use smol_str::SmolStr;

use crate::{MacroGroup,compiler::shader_compiler::ShaderCompiler, package::Package, pkg_inst::PackageInstance};

use super::{IShaderBackend, combinadics::start_combination};




pub fn compile_shader<'a,B:IShaderBackend>(
    package:&'a mut Package,
    shader_name:&str,
    macros:&Vec<SmolStr>,
    out_path:PathBuf,
    compiled:&mut HashSet<u64>,
    backend:&B,ex_data:&B::ExData) -> Option<Arc<Shader>> {
    let efind_shader = package.info.shaders.iter().find(|v| v.name == shader_name);
    if efind_shader.is_none() {
        log::error!("not found shader {} in package {}",shader_name,package.info.name);
    }
    
    let find_shader = efind_shader?.clone();
    let mut requires:Vec<SmolStr> = vec![];
    let mut options:Vec<SmolStr> = vec![];
    let mut require_verts:Vec<SmolStr> = vec![];
    let mut options_verts:Vec<SmolStr> = vec![];

    for (v_string,is_require) in find_shader.vertexs.iter() {
        let nv:SmolStr = format!("VERTEX_{}",v_string.as_str()).into();
        
        if *is_require {
            require_verts.push(v_string.clone());
            requires.push(nv);
        } else {
            options.push(nv);
            options_verts.push(v_string.clone());
        }
    }
    
    start_combination(options.len(), |idxs| {
        let mut all_macros:Vec<SmolStr> = vec![];
        let mut all_verts:Vec<SmolStr> = vec![];
        for (idx,is_use) in idxs.iter() {
            if *is_use {  
                all_macros.push(options[*idx].clone());
                all_verts.push(options_verts[*idx].clone()); 
            }
        }
        all_macros.extend(macros.iter().map(|v| v.clone()));
        all_macros.extend(requires.iter().map(|v| v.clone()));
        let group = MacroGroup::new(all_macros);
        let pkg_inst = package.get_inst(&group);

        all_verts.extend(require_verts.iter().map(|v| v.clone()));
       
        run_macro(&out_path, pkg_inst.clone(), &find_shader, &group, backend,&all_verts,compiled,ex_data);
    });
    Some(find_shader.clone())
    
}

fn run_macro<B:IShaderBackend>(out_path:&PathBuf,
                               pkg_inst:Arc<PackageInstance>,
                               shader:&Arc<Shader>,
                               macros:&MacroGroup,
                               backend:&B,verts:&Vec<SmolStr>,
                               compiled:&mut HashSet<u64>,ex_data:&B::ExData) {
    let macro_hash = macros.hash_base64();
   
    let mut vs_string = String::default();
    let mut fs_string:String  = String::default();
    let mut shader_compiler = ShaderCompiler::new(shader.clone(),pkg_inst.clone());
    shader_compiler.compile(backend,&mut vs_string,&mut fs_string,verts,ex_data);
    let vs_file_name = format!("{}#{}_{}.vert",pkg_inst.info.name,shader.name,macro_hash); 
    let fs_file_name = format!("{}#{}_{}.frag",pkg_inst.info.name,shader.name,macro_hash);
    
    let mut hasher = DefaultHasher::default();
    vs_file_name.hash(&mut hasher);
    vs_string.hash(&mut hasher);
    let vs_hash_code = hasher.finish();

    hasher = DefaultHasher::default();
    fs_file_name.hash(&mut hasher);
    fs_string.hash(&mut hasher);
    let fs_hash_code = hasher.finish();
    
    let (has_vs,has_fs) = (compiled.contains(&vs_hash_code),compiled.contains(&fs_hash_code));

    let mut compiler:Option<Compiler> = None;
    if !has_vs || !has_fs {  compiler = Some(shaderc::Compiler::new().unwrap()); }
    
    if !has_vs {
        std::fs::write(out_path.join(&vs_file_name), &vs_string).unwrap();
        let rvert_spv = compiler.as_mut().unwrap()
                                                                .compile_into_spirv(&vs_string,
                                                                shaderc::ShaderKind::Vertex,
                                                                &vs_file_name,
                                                                "main", None);
        if let Err(err) = rvert_spv {
            log::error!("{} error:{:?}",&vs_file_name,&err);
            return;
        }
        std::fs::write(out_path.join( format!("{}.spv",&vs_file_name)), &rvert_spv.unwrap().as_binary_u8()).unwrap();
        log::info!("write {}",&vs_file_name);
        compiled.insert(vs_hash_code);
    }
    
    if !has_fs {
        std::fs::write(out_path.join(&fs_file_name), &fs_string).unwrap();
        let rfrag_spv = compiler.as_mut().unwrap()
                                                                .compile_into_spirv(&fs_string,
                                                                                 shaderc::ShaderKind::Fragment,
                                                                &fs_file_name,
                                                                "main", None);
        if let Err(err) = rfrag_spv {
            log::error!("{} error:{:?}",&fs_file_name,&err);
            return;
        }
        std::fs::write(out_path.join(format!("{}.spv",&fs_file_name)), &rfrag_spv.unwrap().as_binary_u8()).unwrap();
        log::info!("write {}",&fs_file_name);
        compiled.insert(fs_hash_code);
    } 
}