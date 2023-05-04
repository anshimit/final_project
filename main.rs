use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

mod module;

use crate::module::mod_graph::Graph;
use crate::module::mod_graph::Node;


fn read_file(filename: &str) -> Graph {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut graph = Graph::new();
    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        match parts[..] {
            [_, source, target, weight, book,..] => {
                graph.add_edge(source, target, weight.parse().unwrap(), book.parse().unwrap());
            }
            _ => panic!("Invalid line: {}", line),
        }
    }
    graph
}

fn max_neighbors(graph: &Graph) {
    let mut max_neighbors_count = 0;
    let mut max_neighbors_node: Option<Rc<Node>> = None;

    for node in &graph.nodes {
        let neighbors_count = graph.next_node(node).len();
        if neighbors_count > max_neighbors_count {
            max_neighbors_count = neighbors_count;
            max_neighbors_node = Some(node.clone());
        }
    }

    if let Some(node) = max_neighbors_node {
        println!("{} has the most neighbors with {}", node.name, max_neighbors_count);
    }
}

fn node_neighbors_count(graph: &Graph) {
    for node in &graph.nodes {
        let neighbors_count = graph.next_node(node).len();

        let mut neighbors: Vec<(Rc<Node>, i32, i32)> = graph.edges.iter()
            .filter(|e| Rc::ptr_eq(&e.source, node))
            .map(|e| (e.target.clone(), e.weight, e.book))
            .collect();

        neighbors.sort_by_key(|(_, weight, _)| -*weight);

        println!("{} interacts with {} neighbors:", node.name, neighbors_count);
        for (neighbor, weight, book) in neighbors {
            println!("- {} (weight {}) in book {}", neighbor.name, weight, book);
        }
    }
}

fn main() {
    let graph = read_file("witcher_network.csv");
    node_neighbors_count(&graph);
    max_neighbors(&graph);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_graph() -> Graph {
        let mut graph = Graph::new();
        graph.add_node("Alice");
        graph.add_node("Bob");
        graph.add_node("Cat");
        graph.add_edge("Alice", "Bob", 1, 2);
        graph.add_edge("Bob", "Cat", 3, 4);

    graph
    }   

    #[test]
    fn test_node_neighbors_count() {
        let graph = test_graph();
        node_neighbors_count(&graph)    
    }

    #[test]
    fn test_max_neighbors() {
        let graph = test_graph();
        max_neighbors(&graph)
    }

}
