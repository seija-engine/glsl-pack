use std::{sync::Arc, fmt::Write};
use glsl_lang::transpiler::glsl::{FormattingState, show_external_declaration};

use crate::{ast::{SymbolName, RcSymbolName}, pkg_inst::PackageInstance};

use super::{ dag::{Graph, NodeId}, DepSearch};


pub struct SymbolGenerator {
    inst:Arc<PackageInstance>,
    //symbols:Vec<Vec<SymbolName>>,
    pub fs:FormattingState<'static>,
    graph:Graph<RcSymbolName>, 
    dep_search:DepSearch
}

impl SymbolGenerator {
    pub fn new(inst:Arc<PackageInstance>) -> Self {
        SymbolGenerator { 
            inst,
            //symbols:vec![],
            fs:FormattingState::default(),
            graph:Graph::new(),
            dep_search:DepSearch::new() 
        }
    }

    pub fn run<W:Write>(&mut self,symbol:&SymbolName,writer:&mut W,out_self:bool) {
        let rid = self.graph.add(symbol.to_owned().into());
       
        self.search_step(&rid);

        if let Ok(lst) = self.graph.sort() {
            for node_id in lst.iter().rev() {
                if !out_self && node_id.0 == 0 { continue; }
                let node = self.graph.get(node_id);
                if let Some((decl,_)) = self.inst.ast_pkg.find_decl(&node.value.0) {
                    show_external_declaration(writer, decl, &mut self.fs).unwrap();
                }
            }
        }
    }

    fn search_step(&mut self,src_node_id:&NodeId) {
        let node = self.graph.get(&src_node_id);
        for search_sym in self.dep_search.search(&node.value.0, &self.inst) {
            let new_node_id = self.get_or_add(search_sym);
            self.graph.add_link(*src_node_id, new_node_id);

            self.search_step(&new_node_id);
        }
    }

    fn get_or_add(&mut self,sym:RcSymbolName) -> NodeId {
        if let Some(id) = self.graph.caches.get(&sym) {
            *id
        } else {
            self.graph.add(sym.clone())
        }
    }
}