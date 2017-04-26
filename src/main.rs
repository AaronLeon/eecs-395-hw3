use std::env;
use std::fs::File;
use std::io::{stdin, Read};

mod graph;
use graph::Graph;

fn main() {
    let nodes = read_graph_file();
    //let g = generate_graph(nodes);
    let mut g = Graph::new();
    let a = "a".to_string();
    let b = "b".to_string();
    let c = "c".to_string();
    let d = "d".to_string();
    let e = "e".to_string();
    g.add_node(&a);
    g.add_node(&b);
    g.add_node(&c);
    g.add_node(&d);
    g.add_node(&e);
    g.add_edge(&a, &b);
    g.add_edge(&a, &d);
    g.add_edge(&c, &e);
    g.add_edge(&d, &b);
    g.add_edge(&e, &d);

    let expected1 = vec![a.to_owned(), b.to_owned(), d.to_owned()];
    let expected2 = vec![a.to_owned(), d.to_owned()];
    let path = g.get_path(&a, &c);
    match path {
        Some(p) => print!("{:?}", p),
        None => print!("NONE!!"),
    }
}

fn generate_graph(nodes:Vec<Vec<String>>) -> graph::Graph {
    let mut g:Graph = Graph::new();

    for node in nodes.iter() {
        let mut node_iter = node.iter();
        let first = node_iter.next().unwrap();
        g.add_node(&first);
        for value in node_iter {
            g.add_edge(&first, &value.to_string());
        }
    };

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

fn read_graph_file() -> Vec<Vec<String>> {
    let file_name = env::args().nth(1).unwrap();
    let mut f = File::open(file_name).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).ok();
    // buffer
    let mut result:Vec<Vec<String>> = Vec::new();

    for line in buffer.split('\n') {
        let mut temp:Vec<String> = Vec::new();
        for token in line.split(' ') {
            temp.push(token.to_string());
        }
        result.push(temp);
    }

    result
}

#[cfg(test)]
mod path_finder_test {
    use super::{read_graph_file, generate_graph};

}
