use std::{fmt::Write, sync::Arc};

use glsl_lang::{ast::*, transpiler::glsl::show_function_definition};

use crate::{pkg_inst::PackageInstance, compiler::sym_generator::SymbolGenerator, ast::SymbolName};

use super::ReplaceReturnStmt;

pub fn run_fs_dep_main_step<W:Write>(main_name:&str,inst:Arc<PackageInstance>,writer:&mut W,sym:Option<SymbolName>) {
    let mut sym_gen = SymbolGenerator::new(inst.clone());
    let main_sym_name = SymbolName::parse(main_name);
    sym_gen.run(&main_sym_name,writer,false);

    if let Some(in_sym) = sym {
        writer.write_str("\r\n").unwrap();
        writer.write_fmt(format_args!("layout(location = 0) in {} _input;\r\n",&in_sym.name)).unwrap();
    }

    if let Some((decl,_file)) = inst.ast_pkg.find_decl(&main_sym_name) {
        match &decl.content {
            ExternalDeclarationData::FunctionDefinition(fd) => {
                let new_func = re_generator_function(fd);
                writer.write_str("\r\n").unwrap();
                show_function_definition(writer, &new_func, &mut sym_gen.fs).unwrap();
            },
            _ => {
               log::error!("{:?} must be function",main_name);
            }
         }   
    }
}

fn find_fst_param_name(fd:&FunctionDefinition) -> String {
    for param in fd.prototype.parameters.iter() {
        match &param.content {
            FunctionParameterDeclarationData::Named(_,named) => {
               return named.ident.ident.0.to_string();
            },
            _ => {}
        }
    }
    "_input".to_string()
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

fn re_generator_function(old_decl:&FunctionDefinition) -> FunctionDefinition {
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
    let decl_stmt = make_decl("_input", param_name.as_str(), "VSOutput");
    

    let mut re_gen = ReplaceReturnStmt::default();
    re_gen.replace_name("_outColor");
    let mut new_stmt = re_gen.rum_compound_stmt(&old_decl.statement);
    new_stmt.statement_list.insert(0, decl_stmt);
    FunctionDefinitionData {
      prototype:new_fn_prototype.into(),
      statement:new_stmt
    }.into()
}