use std::collections::HashSet;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq)]
struct Node<T> where T: Hash + PartialEq + Eq {
    value: T,
    neighbors: Vec<Node<T>>,
}

pub struct Graph<T> where T: Hash + PartialEq + Eq {
    nodes: HashSet<Node<T>>
}


impl<T> Graph<T> where T: Hash + PartialEq + Eq {
    fn new() -> Graph<T> {
        Graph {
            nodes: HashSet::new()
        }
    }

    fn add(&self, new_node:Node<T>) {
        &self.nodes.insert(new_node);
    }

    
}

impl<T> Node<T> where T: Hash + PartialEq  + Eq {
    fn new(value:T, neighbors:Vec<Node<T>>) -> Node<T> {
        Node {
            value: value,
            neighbors: neighbors,
        }
    }

    fn add_neighbor(&self, new_neighbor:Node<T>) {
        if !&self.neighbors.contains(&new_neighbor) {
            if !new_neighbor.neighbors.contains(&self) {
                new_neighbor.neighbors.push(self);
            }
            &self.neighbors.push(new_neighbor); 
        }
    }
}
