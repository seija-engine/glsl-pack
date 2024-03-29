use std::{collections::{HashSet, HashMap}, hash::{Hash}};

#[derive(Debug)]
pub struct Graph<T:Hash + Clone + Eq> {
    nodes:Vec<Node<T>>,
    pub caches:HashMap<T,NodeId>
}



#[derive(Debug)]
pub struct Node<T> {
    pub id:NodeId,
    pub value:T,
    pub inputs:Vec<Link>,
    pub outputs:Vec<Link>
}

impl<T> Graph<T> where T:Hash + Clone + Eq {
    pub fn new() -> Graph<T> {
       
        Graph { nodes: vec![], caches: HashMap::default() }
    }

    pub fn add(&mut self,value:T) -> NodeId {
        let node_id = NodeId(self.nodes.len());
        let new_node = Node::new(value,node_id);
        let clone_t = new_node.value.clone();
        self.nodes.push(new_node);
        self.caches.insert(clone_t, node_id);
        node_id
    }

    

    pub fn get(&self,id:&NodeId) -> &Node<T> {
        self.nodes.get(id.0).unwrap()
    }

    pub fn add_link(&mut self,form:NodeId,to:NodeId) {
        let link = Link::new(form, to);
        if let Some(form_node) = self.nodes.get_mut(form.0) {
            form_node.outputs.push(link);
        }
        if let Some(to_node) = self.nodes.get_mut(to.0) {
            to_node.inputs.push(link);
        }
    }
    
    
    pub fn sort(&mut self) -> Result<Vec<NodeId>,()>  {
        let mut ret_list:Vec<NodeId> = vec![];
        let mut out_sets:HashSet<usize> = HashSet::default();
        loop {
            let mut has_new = false;
            for (idx,node) in self.nodes.iter().enumerate() {
                if !out_sets.contains(&idx) {
                    if node.inputs.iter().all(|l| out_sets.contains(&l.form.0)) {
                        has_new = true;
                        out_sets.insert(idx);
                        ret_list.push(NodeId(idx));
                    }
                }
            }
            if !has_new {
                if ret_list.len() == self.nodes.len() {
                    return Ok(ret_list);
                } else {
                    return Err(());
                }
            }
        }
    }
}

#[derive(Clone, Copy,Debug,Hash,PartialEq, PartialOrd)]
pub struct NodeId(pub usize);

#[derive(Clone, Copy,Debug)]
#[allow(dead_code)]
pub struct Link {
    form:NodeId,
    to:NodeId
}

impl Link {
    pub fn new(form:NodeId,to:NodeId) -> Self {
        Link { form, to }
    }
}



impl<T:Hash> Node<T> {
    pub fn new(t:T,id:NodeId) -> Self {
        Node { value: t, inputs: vec![],outputs:vec![],id }
    }
}


#[test]
fn test_graph() {
    let mut graph:Graph<String> = Graph::new();
    
    let node_1 = graph.add("1".into());
    let node_2 = graph.add("2".into());
    let node_3 = graph.add("3".into());
    let node_4 = graph.add("4".into());
    let node_5 = graph.add("5".into());

    graph.add_link(node_1, node_2);
    graph.add_link(node_1, node_4);
    graph.add_link(node_2, node_4);
    graph.add_link(node_2, node_3);
    graph.add_link(node_3, node_5);
    graph.add_link(node_4, node_3);
    graph.add_link(node_4, node_5);

    let ids =  graph.sort();
    dbg!(ids.unwrap());
}