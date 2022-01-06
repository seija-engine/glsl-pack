use std::{sync::Arc, marker::PhantomData};
use std::fmt::Write;
use glsl_lang::ast::*;
use glsl_lang::transpiler::glsl::{self as glsl_t, FormattingState};

use crate::ast::ASTFile;
use crate::{pkg_inst::PackageInstance, ast::SymbolName};

use super::compile_env::CompileEnv;

