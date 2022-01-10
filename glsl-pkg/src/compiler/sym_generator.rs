use std::sync::Arc;

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

    pub fn run(&mut self,symbols:Vec<SymbolName>) {
        self.symbols.push(symbols);
        let mut dep_search = DepSearch::new();

        let mut search_symbols:&Vec<SymbolName> = self.symbols.last().unwrap();
        loop {
            let new_symbols = dep_search.search(search_symbols, &self.inst);
            
            self.symbols.push(new_symbols);
            search_symbols = self.symbols.last().unwrap();
            if search_symbols.len() == 0 { break }
        }

        let mut out:String = String::default();
        for symbol_lst in self.symbols.iter().rev() {
            for symbol in symbol_lst.iter().rev() {
                if let Some((decl,_)) = self.inst.ast_pkg.find_decl(symbol) {
                    show_external_declaration(&mut out, decl, &mut self.fs).unwrap();
                } else {
                    log::warn!("not found:{:?}",symbol);
                }
            }
        }

        std::fs::write(self.inst.info.path.join("testOut.glsl"), out);
    }
}




#[test]
fn load_package() {
    use crate::package::Package;
    use crate::MacroGroup;

    env_logger::init();
    let mut pkg = Package::load("../tests/core/").unwrap();
    let macros = &MacroGroup::new(vec!["HAS_POSITION".to_string()]);
    let sym_vs = SymbolName::parse("color.vs_main");


    let inst = pkg.get_inst(macros);
    let mut generator = SymbolGenerator::new(inst);
    generator.run(vec![sym_vs]);

    

}