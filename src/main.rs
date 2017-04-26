use std::env;
use std::fs::File;
use std::io::{stdin, Read, stdout, Write};

mod graph;
use graph::Graph;

fn main() {
    let nodes = read_graph_file();
    let g = generate_graph(nodes);
    print_paths(stdout(), read_input(stdin()), &g);
}

fn print_paths<W: Write>(mut writer:W, queries:Vec<String>, graph:&Graph) {
    for query in queries {
        let q:Vec<String> = query.split(' ')
            .map(|c| c.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        writeln!(writer, "-> {}", query).ok();
        if let Some(p) = graph.get_path(&q[0], &q[1]) {
            let res = p.join(" ");
            write!(writer, "{}\n", res).ok();
        } else {
            writeln!(writer, "No such path exists").ok();
        }
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

    let result:Vec<String> = buffer.split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
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

//#[cfg(test)]
//mod path_finder_test {
    //use super::{read_graph_file, generate_graph};

//}
