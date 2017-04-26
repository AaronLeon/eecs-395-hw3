use std::env;
use std::fs::File;
use std::io::{stdin, Read};
use std::collections::{HashMap, HashSet};

mod graph;

fn main() {
    let train_file_nodes:Vec<String> = read_train_file();
    G = graph::new();
    let mut empty_neighbors = Vec::new();
    for node in train_file_nodes{
        n = Node::new(node, empty_neighbors);
        G::add(n);
        //  For all the remaining nodes in the line we add them as neighbors
        n.add_neighbor(non_current_node);
    }
}

fn read_input<R: Read>(mut reader: R) -> Vec<String> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).ok();
    
    let result:Vec<String> = buffer.split_whitespace()
        .map(|s| s.to_string())
        .collect();
    result
}

fn read_train_file() -> Vec<String> {
    let file_name = env::args().nth(1).unwrap();
    let mut f = File::open(file_name).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).ok();
    // buffer
    let result:Vec<String> = buffer.split_whitespace()
        .map(|s| s.to_string())
        .collect();

    result
}

fn make_graph(){
    println!("Make the graph");
    // Read the input from stdin line by line
    // Split on whitespaces, and iterate over the data adding a node to the graph one at a time

    
}