use std::{path::{PathBuf, Path}, fs, sync::Arc};

use glsl_pack_rtbase::shader::Shader;

use crate::{MacroGroup, CompileEnv, compiler::shader_compiler::ShaderCompiler, package::Package, pkg_inst::PackageInstance};

use super::{IShaderBackend, combinadics::start_combination};




pub fn compile_shader<'a,B:IShaderBackend>(package:&'a mut Package,shader_name:&str,macros:&Vec<String>,out_path:PathBuf,backend:&B) -> Option<Arc<Shader>> {
    let macro_group = MacroGroup::new(macros.clone());
    let pkg_inst = package.get_inst(&macro_group);
    let find_shader = pkg_inst.info.shaders.iter().find(|v| v.name == shader_name);
    match find_shader {
        Some(shader) => {
            let mut const_macros = macro_group.names.clone();
            let mut requires:Vec<String> = vec![];
            let mut options:Vec<String> = vec![];
            for (v,is_require) in shader.vertexs.iter() {
                let mut nv = "VERTEX_".to_string();
                nv.push_str(v.as_str());
                if *is_require {
                    requires.push(nv);
                } else {
                    options.push(nv);
                }
            }
            const_macros.extend(requires);

            start_combination(options.len(), |idxs| {
                let mut all_macros:Vec<String> = vec![];
                for (idx,is_use) in idxs.iter() {
                    if *is_use {
                        all_macros.push(options[*idx].clone());
                    }
                }
                all_macros.extend(const_macros.clone());
                run_macro(&out_path, pkg_inst.clone(), shader, &MacroGroup::new(all_macros), backend);
                
            });
            Some(shader.clone())
        },
        None => {
            log::error!("not found shader {} in package {}",shader_name,package.info.name);
            None
        }
    }
}

fn run_macro<B:IShaderBackend>(out_path:&PathBuf,pkg_inst:Arc<PackageInstance>,shader:&Arc<Shader>,macros:&MacroGroup,backend:&B) {
    let macro_hash = macros.hash_base64();
   
    let mut vs_string = String::default();
    let mut fs_string:String  = String::default();
    let mut shader_compiler = ShaderCompiler::new(shader.clone(),pkg_inst.clone());
    shader_compiler.compile(backend,&mut vs_string,&mut fs_string);
    let vs_file_name = format!("{}#{}_{}.vert",pkg_inst.info.name,shader.name,macro_hash); 
    let fs_file_name = format!("{}#{}_{}.frag",pkg_inst.info.name,shader.name,macro_hash);
    

    let mut compiler = shaderc::Compiler::new().unwrap();
  
    let rvert_spv = compiler.compile_into_spirv(&vs_string,shaderc::ShaderKind::Vertex,&vs_file_name, "main", None);
    let rfrag_spv = compiler.compile_into_spirv(&fs_string,shaderc::ShaderKind::Fragment,&fs_file_name, "main", None);
    if let Err(err) = rvert_spv {
        log::error!("{} error:{:?}",&vs_file_name,&err);
        return;
    }
    if let Err(err) = rfrag_spv {
        log::error!("{} error:{:?}",&fs_file_name,&err);
        return;
    }
    
    std::fs::write(out_path.join( format!("{}.spv",&vs_file_name)), &rvert_spv.unwrap().as_binary_u8()).unwrap();
    std::fs::write(out_path.join(format!("{}.spv",&fs_file_name)), &rfrag_spv.unwrap().as_binary_u8()).unwrap();

    std::fs::write(out_path.join(&vs_file_name), vs_string).unwrap();
    std::fs::write(out_path.join(&fs_file_name), fs_string).unwrap();
}
