use std::collections::HashMap;
use std::collections::HashSet;

pub struct Graph {
    pub nodes: HashMap<String, HashSet<String>>
}

impl Graph {
    pub fn new() -> Self {
        let graph = Graph {
            nodes: HashMap::new()
        };
        graph
    }

    pub fn add_node(&mut self, value:&String) {
        let neighbors = HashSet::new();
        self.nodes.insert(value.to_owned(), neighbors);
    } 

    pub fn add_edge(&mut self, a:&String, b:&String) {
        if !self.nodes.contains_key(a) {
            self.add_node(a)
        } else if !self.nodes.contains_key(b) {
            self.add_node(b)
        }
        self.nodes.get_mut(a).unwrap().insert(b.to_owned());
        self.nodes.get_mut(b).unwrap().insert(a.to_owned());
    }

    pub fn get_neighbors(&self, value:&String) -> Option<&HashSet<String>> {
        self.nodes.get(value)
    }

    pub fn get_path(&self, a:&String, b:&String) -> Option<Vec<String>> {
        let visited = HashSet::new();
        let path = Vec::new();
        let res = self.get_path_helper(a, b, &visited, &path);
        res
    }

    fn get_path_helper(&self, a:&String, b:&String, visited:&HashSet<String>, path:&Vec<String>) -> Option<Vec<String>> {
        let mut v = visited.clone();
        let mut p = path.clone();
        v.insert(a.to_owned());
        p.push(a.to_owned());
        if *&a == *&b {
            return Some(p.clone());
        }
        if let Some(neighbors) = self.get_neighbors(a) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if let Some(res) = self.get_path_helper(neighbor, b, &v, &p) {
                        return Some(res);
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod graph_tests {
    use super::Graph;

    #[test]
    fn new_graph_is_empty_test() {
        let g = Graph::new();
        assert!(g.nodes.is_empty());
    }

    #[test]
    fn add_node_creates_node_test() {
        let mut g = Graph::new();
        let value = "a".to_string();
        g.add_node(&value);
        assert!(g.nodes.contains_key(&value));
        assert!(g.nodes.get(&value).unwrap().is_empty());
    }

    #[test]
    fn get_neighbors_returns_neighbors_test() {
        let mut g = Graph::new();
        let value = "a".to_string();
        let neighbor = "b".to_string();
        g.add_node(&value);
        g.nodes.get_mut(&value).unwrap().insert((&neighbor).to_owned());
        assert!(g.get_neighbors(&value).unwrap().contains(&neighbor));
    }

    #[test]
    fn add_edge_appends_to_both_neighbors_test() {
        let mut g = Graph::new();
        let a = "a".to_string();
        let b = "b".to_string();
        g.add_node(&a);
        g.add_edge(&a, &b);
        assert!(g.get_neighbors(&a).unwrap().contains(&b));
        assert!(g.get_neighbors(&b).unwrap().contains(&a));
    }

    #[test]
    fn get_path_of_adjacent_nodes_test() {
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

        let expected1 = vec![a.to_owned(), d.to_owned(), b.to_owned()];
        let expected2 = vec![a.to_owned(), b.to_owned()];
        let path = g.get_path(&a, &b).unwrap();
        assert!(path == expected1 || path == expected2);
    }

    #[test]
    fn get_path_of_distant_nodes_test() {
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

        let expected1 = vec![a.to_owned(), b.to_owned(), d.to_owned(), e.to_owned(), c.to_owned()];
        let expected2 = vec![a.to_owned(), d.to_owned(), e.to_owned(), c.to_owned()];
        let path = g.get_path(&a, &c).unwrap();
        assert!(path == expected1 || path == expected2);
    }

    #[test]
    fn get_path_of_linear_path_test() {
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
        g.add_edge(&b, &c);
        g.add_edge(&c, &d);
        g.add_edge(&d, &e);

        let expected = vec![a.to_owned(), b.to_owned(), c.to_owned(), d.to_owned(), e.to_owned()];
        assert!(g.get_path(&a, &e).unwrap() == expected);
    }

    #[test]
    fn get_path_of_same_node_test() {
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

        let expected = vec![a.to_owned()];
        let path = g.get_path(&a, &a).unwrap();
        assert!(path == expected);

    }

    #[test]
    fn get_path_when_no_path_exists_test() {
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
        g.add_edge(&d, &b);
        g.add_edge(&e, &d);

        assert!(g.get_path(&a, &c).is_none());
    }
}
