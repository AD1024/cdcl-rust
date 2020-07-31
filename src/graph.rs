use crate::AST;
use std::collections::HashMap;
use std::borrow::BorrowMut;

#[derive(Clone)]
#[derive(Debug)]
pub struct GraphNode {
    formula : AST,
    neighbor : Vec<GraphNode>
}

pub struct Graph {
    node_map: HashMap<String, GraphNode>
}

impl Graph {
    fn new() -> Self {
        Graph { node_map : HashMap::new() }
    }

    fn add_edge(&mut self, from : &str, to : &GraphNode) {
        if let Some(mut v) = self.node_map.get_mut(from) {
            v.neighbor.push(to.clone());
        } else {
            panic!("Does not contain key: {}", from);
        }
    }
}