use std::{fmt::Write, sync::Arc};

use glsl_lang::{ast::*, transpiler::glsl::{show_function_definition}};


use crate::{pkg_inst::PackageInstance, compiler::sym_generator::SymbolGenerator, ast::SymbolName};

use super::ReplaceReturnStmt;

pub fn run_fs_dep_main_step<F:FnOnce(&mut W),W:Write>(main_name:&str,inst:Arc<PackageInstance>,writer:&mut W,input_sym:Option<SymbolName>,f:F) {
    let mut sym_gen = SymbolGenerator::new(inst.clone());
    let main_sym_name = SymbolName::parse(main_name);
    sym_gen.run(&main_sym_name,writer,false);

    if let Some(in_sym) = input_sym.as_ref() {
        writer.write_str("\r\n").unwrap();
        writer.write_fmt(format_args!("layout(location = 0) in {} _input;\r\n",&in_sym.name)).unwrap();
    }
    f(writer);

    if let Some((decl,_file)) = inst.ast_pkg.find_decl(&main_sym_name) {
        match &decl.content {
            ExternalDeclarationData::FunctionDefinition(fd) => {
                if let TypeSpecifierNonArrayData::TypeName(type_name)  = &fd.prototype.ty.ty.ty.content {
                   let mut type_sym = main_sym_name.clone();
                   type_sym.name = type_name.as_str().to_string();
                   let find_type = inst.ast_pkg.find_decl(&type_sym);
                   if let Some(struct_type) = find_type {
                       re_generator_struct_function(writer,fd,struct_type.0,&mut sym_gen,input_sym.as_ref());
                   } else {
                       log::error!("not found type:{:?}",&type_sym);
                   }
                } else {
                    writer.write_str("layout(location = 0) out vec4 _outColor;\r\n").unwrap();
                    let new_func = re_generator_function(fd,input_sym.as_ref());
                    writer.write_str("\r\n").unwrap();
                    show_function_definition(writer, &new_func, &mut sym_gen.fs).unwrap();
                }  
            },
            _ => {
               log::error!("{:?} must be function",main_name);
            }
         }   
    }
}

fn re_generator_struct_function<W:Write>(write:&mut W,old_func:&FunctionDefinition,
                                         struct_decl:&ExternalDeclaration,sym_gen:&mut SymbolGenerator,
                                         in_type_sym:Option<&SymbolName>) {
    let (attrs,struct_name) = find_struct_attrs(struct_decl);
    let mut index = 0;
    for (typ,name) in attrs.iter() {
        let type_str = match typ.content {
            TypeSpecifierNonArrayData::Vec4 => {"vec4"},
            TypeSpecifierNonArrayData::Vec3 => {"vec3"},
            _ => {
                log::error!("error fs output type:{:?}",typ);
                return;
            },
        };
        write.write_str(&format!("layout(location = {}) out {} _fs_output_{};\r\n",index,type_str,name.as_str())).unwrap();
        index += 1;
    }
    
    let mut rename_old_func = old_func.clone();
    rename_old_func.content.prototype.name.0 = "_muli_main".into();
    let in_param = rename_old_func.prototype.parameters[0].clone();
    let in_param_name = match &in_param.content {
        FunctionParameterDeclarationData::Named(_,namded) => {
            namded.ident.ident.as_str().to_string()
           
        },
        _ => { 
            String::default()
        },
    };
    let fst_decl = make_decl("_input", in_param_name.as_str(), &in_type_sym.unwrap().name);
    rename_old_func.content.statement.content.statement_list.insert(0, fst_decl);

    rename_old_func.prototype.parameters.clear();
    show_function_definition(write, &rename_old_func, &mut sym_gen.fs).unwrap();
   
    
    //show_type_specifier_non_array(write, &attrs[0].0, &mut sym_gen.fs);
    //dbg!(&attrs);
    write.write_str("void main() {\r\n").unwrap();
    write.write_str(&format!("  {} value = _muli_main();\r\n",struct_name.as_str())).unwrap();
    for (_,name) in attrs {
        write.write_str(&format!("  _fs_output_{} = value.{};\r\n",name.as_str(),name.as_str())).unwrap();
    }
    write.write_str("\r\n}").unwrap();

}

fn find_struct_attrs(struct_decl:&ExternalDeclaration) -> (Vec<(TypeSpecifierNonArray,String)>,String) {
    let mut attrs:Vec<(TypeSpecifierNonArray,String)> = vec![];
    let mut type_name = String::default();
    match &struct_decl.content {
        ExternalDeclarationData::Declaration(decl) => {
            match &decl.content {
                DeclarationData::InitDeclaratorList(decl_list) => {
                    if let TypeSpecifierNonArrayData::Struct(struct_type) = &decl_list.head.ty.ty.ty.content {
                        type_name = struct_type.name.as_ref().unwrap().as_str().to_string();
                        for filed in struct_type.fields.iter() {
                            let ident_type = filed.ty.content.ty.content.clone();
                            let ident_name = filed.identifiers[0].content.ident.as_str();
                            attrs.push((ident_type.into(),ident_name.to_string()));
                        }
                    }
                   
                },
                _ => {}
            }
        }
        _ => {}
    }
    (attrs,type_name)
}

fn find_fst_param_name(fd:&FunctionDefinition) -> Option<String> {
    for param in fd.prototype.parameters.iter() {
        match &param.content {
            FunctionParameterDeclarationData::Named(_,named) => {
               return Some(named.ident.ident.0.to_string());
            },
            _ => {}
        }
    }
    None
}

fn make_decl(from_name:&str,to_name:&str,type_name:&str) -> Statement {
    let type_name_data:TypeNameData = type_name.into();
    let ts:TypeSpecifier = TypeSpecifierData { ty:TypeSpecifierNonArrayData::TypeName(type_name_data.into()).into(),array_specifier:None }.into();
    let full_type:FullySpecifiedType = FullySpecifiedTypeData {qualifier:None,ty:ts }.into();
    let to_name_data:IdentifierData = to_name.into();

    let from_id_data:IdentifierData = from_name.into();
    let expr_data:Expr = ExprData::Variable(from_id_data.into()).into();
    let initer = InitializerData::Simple(Box::new(expr_data));

    let sdecl = SingleDeclarationData {
        ty:full_type,
        name:Some(to_name_data.into()),
        array_specifier:None,
        initializer:Some(initer.into()),
    };
    let init_decl:InitDeclaratorListData = InitDeclaratorListData {head:sdecl.into(), tail:vec![]}.into();
    let decl:Declaration = DeclarationData::InitDeclaratorList(init_decl.into()).into();

    StatementData::Declaration(decl.into()).into()
}

fn re_generator_function(old_decl:&FunctionDefinition,in_type_sym:Option<&SymbolName>) -> FunctionDefinition {
    let param_name = find_fst_param_name(old_decl);
    let fn_id = Identifier::new(IdentifierData("main".into()), None);
    let full_ty =  FullySpecifiedTypeData {
      qualifier:None,
      ty:TypeSpecifierData {
         ty:TypeSpecifierNonArrayData::Void.into(),
         array_specifier: None,
      }.into()
    };
    let new_fn_prototype = FunctionPrototypeData {
      ty:full_ty.into(),
      name:fn_id,
      parameters:vec![]
    };
    let decl_stmt = if let Some(param_name) = param_name {
        Some(make_decl("_input", param_name.as_str(), &in_type_sym.unwrap().name))
    } else { None };
    

    let mut re_gen = ReplaceReturnStmt::default();
    re_gen.replace_name("_outColor");
    let mut new_stmt = re_gen.rum_compound_stmt(&old_decl.statement);
    if let Some(decl_stmt) = decl_stmt {
        new_stmt.statement_list.insert(0, decl_stmt);
    }
    FunctionDefinitionData {
      prototype:new_fn_prototype.into(),
      statement:new_stmt
    }.into()
}