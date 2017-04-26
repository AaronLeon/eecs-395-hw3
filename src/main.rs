use std::env;
use std::fs::File;
use std::io::{stdin, Read};
use std::collections::{HashMap, HashSet};

mod graph;

fn main() {
    let lines:Vec<String> = read_train_file();
}

fn generate_graph(input_lines:Vec<String>) -> graph::Graph {
    let mut g:graph::Graph = graph::Graph::new();

    input_lines.iter().map(|line| {
        let mut n_iter = line.split_whitespace();
        let first = n_iter.next();
        g.add(first.unwrap().to_string());
        for value in n_iter {
            let neighbors = Vec::new();
            let mut new_neighbor:graph::Node = graph::Node::new(value.to_string(), neighbors);
            let current:&mut graph::Node = g.get(value.to_string());
            current.add_neighbor(&mut new_neighbor);
        }
    });

    g
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
    let result:Vec<String> = buffer.split('\n')
        .map(|line| line.to_string())
        .collect();

    result
}

fn make_graph(){
    println!("Make the graph");
    // Read the input from stdin line by line
    // Split on whitespaces, and iterate over the data adding a node to the graph one at a time


}
