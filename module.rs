pub mod mod_graph {
use std::rc::Rc;

#[derive (Debug)]
pub struct Node {
    pub name: String,
}

impl Node {
    fn new(name: String) -> Self{
        Self{
            name,
        }
    }
}
#[derive (Debug)]
pub struct Edge {
    pub source: Rc<Node>,
    pub target: Rc<Node>,
    pub weight: i32,
    pub book: i32,
}

impl Edge {
    pub fn new(source: Rc<Node>, target: Rc<Node>, weight: i32, book: i32) -> Self {
        Self{
            source,
            target,
            weight,
            book,
        }
    }
}
#[derive (Debug)]
pub struct Graph {
    pub nodes: Vec<Rc<Node>>,
    pub edges: Vec<Edge>
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![]
        }
    }
    pub fn add_node(&mut self, node_name: &str) -> Rc<Node>{
        match self.find_node_by_name(node_name) {
            Some(node) => node.clone(),
            None => {
                let node = Rc::new(Node::new(node_name.to_string()));
                self.nodes.push(node.clone());
                node.clone()
            }
        }
    }
    pub fn add_edge(&mut self, source_node_name: &str, target_node_name: &str, weight: i32, book:i32){
       let source_node = self.add_node(source_node_name);
       let target_node = self.add_node(target_node_name);
       let edge = Edge::new(source_node, target_node, weight, book);
       self.edges.push(edge);

    }
    pub fn find_node_by_name(&self, name: &str) -> Option<&Rc<Node>> {
        self.nodes.iter().find(|n| n.name == name)
    }

    pub fn next_node(&self, node: &Rc<Node>) -> Vec<Rc<Node>>{
        self.edges.iter()
            .filter(|edge| edge.source.name == node.name)
            .map(|edge| {
                let  target = &edge.target.clone();
                Rc::new(Node{
                   name: target.name.clone(),
                })
            })
            .collect()
    }

    }
}