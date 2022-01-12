use std::{sync::Arc, fmt::Write};

use glsl_lang::transpiler::glsl::{FormattingState, show_external_declaration};

use crate::{ast::SymbolName, pkg_inst::PackageInstance};

use super::DepSearch;

#[derive(Debug)]
pub struct SymbolGenerator {
    inst:Arc<PackageInstance>,
    symbols:Vec<Vec<SymbolName>>,
    fs:FormattingState<'static>
}

impl SymbolGenerator {
    pub fn new(inst:Arc<PackageInstance>) -> Self {
        SymbolGenerator { inst,symbols:vec![],fs:FormattingState::default() }
    }

    pub fn run<W:Write>(&mut self,symbols:Vec<SymbolName>,writer:&mut W) {
        let mut dep_search = DepSearch::new();
        let mut search_symbols:&Vec<SymbolName> = &symbols;
        loop {
            let new_symbols = dep_search.search(search_symbols, &self.inst);
            
            self.symbols.push(new_symbols);
            search_symbols = self.symbols.last().unwrap();
            if search_symbols.len() == 0 { break }
        }


        for symbol_lst in self.symbols.iter().rev() {
            for symbol in symbol_lst.iter().rev() {
                if let Some((decl,_)) = self.inst.ast_pkg.find_decl(symbol) {
                    show_external_declaration(writer, decl, &mut self.fs).unwrap();
                } else {
                    log::warn!("not found:{:?}",symbol);
                }
            }
        }

        
    }
}



