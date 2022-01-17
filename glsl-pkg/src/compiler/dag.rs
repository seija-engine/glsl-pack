use std::{collections::{HashSet, HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

#[derive(Debug)]
pub struct Graph<T:Hash> {
    nodes:Vec<Node<T>>,
    pub caches:HashMap<u64,NodeId> //TODO 考虑优化一下
}



#[derive(Debug)]
pub struct Node<T> {
    pub id:NodeId,
    pub value:T,
    pub inputs:Vec<Link>,
    pub outputs:Vec<Link>
}

impl<T> Graph<T> where T:Hash {
    pub fn new() -> Graph<T> {
       
        Graph { nodes: vec![], caches: HashMap::default() }
    }

    pub fn add(&mut self,value:T) -> NodeId {
        let node_id = NodeId(self.nodes.len());
        let new_node = Node::new(value,node_id);
        let hash_u64 = new_node.hash_u64();
        self.nodes.push(new_node);
        self.caches.insert(hash_u64, node_id);
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
pub struct NodeId(usize);

#[derive(Clone, Copy,Debug)]
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

    fn hash_u64(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.value.hash(&mut hasher);
        hasher.finish()
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
    dbg!(ids);
}