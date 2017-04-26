use std::collections::HashSet;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Node<T> where T: Hash + PartialEq + Eq {
    value: T,
    neighbors: Vec<Node<T>>,
}

pub struct Graph<T> where T: Hash + PartialEq + Eq {
    nodes: HashSet<Node<T>>
}


impl<T> Graph<T> where T: Hash + PartialEq + Eq + Clone {
    fn new() -> Graph<T> {
        Graph {
            nodes: HashSet::new()
        }
    }

    fn add(&mut self, mut new_node:Node<T>) {
        if !self.nodes.contains(&new_node){
            self.nodes.insert(new_node);
        }
    }  
}

impl<T> Node<T> where T: Hash + PartialEq  + Eq + Clone {
    fn new(value:T, neighbors:Vec<Node<T>>) -> Node<T> {
        Node {
            value: value,
            neighbors: neighbors,
        }
    }

    fn add_neighbor(&mut self, mut new_neighbor:Node<T>) {
        let current_node: Node<T> = self.clone();
        if !&self.neighbors.contains(&new_neighbor) {
            // Add the node to the current node's edge list
            &self.neighbors.push(new_neighbor); 
            // Then we should add the other side of the edge, by adding the current node in its neighbor's edge list
            // We have to check if the neighbor exists in the graph, if it does then we add data to its edge list
            // otherwise we add a node to the graph. 
            // Graph::add(&new_neighbor);

            if !new_neighbor.neighbors.contains(&self) {
                new_neighbor.neighbors.push(current_node);
            }
        }
    }
}
