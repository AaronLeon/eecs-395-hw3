use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Node {
    value: String,
    neighbors: Vec<Node>,
}

pub struct Graph {
    nodes: HashMap<String, Node>
}


impl Graph {
    pub fn new() -> Graph {
        let mut graph = Graph {
            nodes: HashMap::new()
        };
        graph
    }

    pub fn add(&mut self, value:String) {
        let v = value.clone();
        let new_neighbors = Vec::new();
        let new_node:Node = Node::new(value, new_neighbors);
        self.nodes.insert(v, new_node);
    } 

    pub fn get(&mut self, value:String) -> &mut Node {
        match self.nodes.get_mut(&value) {
            Some(node) => return node,
            None => panic!("no node found!"),
        }
    }
}

impl Node {
    pub fn new(value:String, neighbors:Vec<Node>) -> Node {
        let mut node = Node {
            value: value,
            neighbors: neighbors,
        };
        node
    }

    pub fn add_neighbor(&mut self, new_neighbor: &mut Node) {
        let nn = new_neighbor.clone();
        let current = self.clone();

        // Add the node to the current node's edge list
        if !&self.neighbors.contains(new_neighbor) {
            &self.neighbors.push(nn); 
        }
        // Stringhen we should add the other side of the edge, by adding the current node in its neighbor's edge list
        // We have to check if the neighbor exists in the graph, if it does then we add data to its edge list
        // otherwise we add a node to the graph. 
        // Graph::add(&new_neighbor);
        if new_neighbor.neighbors.contains(self) {
            new_neighbor.neighbors.push(current);
        }
    }
}
