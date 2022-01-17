use std::{sync::Arc, fmt::Write};
use glsl_lang::transpiler::glsl::{FormattingState};

use crate::{ast::SymbolName, pkg_inst::PackageInstance};

use super::{ dag::{Graph, NodeId}, DepSearch};

#[derive(Debug)]
pub struct SymbolGenerator {
    inst:Arc<PackageInstance>,
    symbols:Vec<Vec<SymbolName>>,
    pub fs:FormattingState<'static>,
    graph:Graph<SymbolName>, 
    dep_search:DepSearch
}

impl SymbolGenerator {
    pub fn new(inst:Arc<PackageInstance>) -> Self {
        SymbolGenerator { 
            inst,
            symbols:vec![],
            fs:FormattingState::default(),
            graph:Graph::new(),
            dep_search:DepSearch::new() 
        }
    }

    pub fn run<W:Write>(&mut self,symbol:&SymbolName,writer:&mut W) {
        let rid = self.graph.add(symbol.clone());
       
        self.search_step(&rid);
        if let Ok(lst) = self.graph.sort() {
            for node_id in lst.iter().rev() {
                let node = self.graph.get(node_id);
                dbg!(&node.value);
            }
        }
        
    }

    fn search_step(&mut self,src_node_id:&NodeId) {
        let node = self.graph.get(&src_node_id);
        for search_sym in self.dep_search.search(&node.value, &self.inst) {
            let new_node_id = self.get_or_add(&search_sym);
            self.graph.add_link(*src_node_id, new_node_id);

            self.search_step(&new_node_id);
        }
    }

    fn get_or_add(&mut self,sym:&SymbolName) -> NodeId {
        let sym_hash = sym.hash_u64();
        if let Some(id) = self.graph.caches.get(&sym_hash) {
            *id
        } else {
            self.graph.add(sym.clone())
        }
    }
}



 /*
        let mut dep_search = DepSearch::new();
        let mut search_symbols:&Vec<SymbolName> = &symbols;
        let mut all_sets:HashSet<SymbolName> = HashSet::default();
        loop {
            let mut new_symbols = dep_search.search(search_symbols, &self.inst);
            let mut real_symbols:Vec<SymbolName> = vec![];
            for sym in new_symbols.drain(..) {
                if !all_sets.contains(&sym) {
                    all_sets.insert(sym.clone());
                    real_symbols.push(sym);
                }
            }
            
            self.symbols.push(real_symbols);
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
        */