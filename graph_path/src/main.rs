use std::io::{stdin, Read, BufReader, BufRead};
use std::fmt;

type NameTable  = std::collections::HashMap<String, usize>;

struct Node {
    name:       String,
    distance:   usize,
    neighbours: Vec<usize>,
}

struct Graph {
    nodes:      Vec<Node>,
    name_table: NameTable,
}

impl Node {
    fn new(name: String) -> Self {
        Node {
            name:       name,
            distance:   usize::max_value(),
            neighbours: Vec::new()
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str("Node ");
        output.push_str(&self.name);
        output.push('\n');
        output.push_str("\tdistance: ");
        output.push_str(&self.distance.to_string());
        output.push('\n');
        output.push_str("\tneighbours: ");
        for n in self.neighbours.iter() {
            output.push_str(&n.to_string());
            output.push(' ');
        }
        output.push('\n');
        write!(f, "{}", output)
    }
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes:      Vec::new(),
            name_table: NameTable::new()
        }
    }
}

fn read_graph<R:Read>(reader: R) -> Graph {
    let mut graph = Graph::new();
    let mut lines = BufReader::new(reader).lines();
    let mut count = 0usize;

    while let Some(Ok(line)) = lines.next() {
        let mut words   = line.split(" ");
        let mut is_node = true; 
        let mut node = 0usize;
        while let Some(word) = words.next() {
            if graph.name_table.get(word).is_none() {
                graph.name_table.insert(word.to_owned(), count);
                graph.nodes.push(Node::new(word.to_owned()));
                count += 1;
            }
            if is_node {
                node = *graph.name_table.get(word).unwrap();
                is_node = false;
            } else {
                let n = *graph.name_table.get(word).unwrap();
                if graph.nodes[node].neighbours.contains(&n) {
                    graph.nodes[node].neighbours.push(n);
                }
                if graph.nodes[n].neighbours.contains(&node) {
                    graph.nodes[n].neighbours.push(node);
                }
            }
        }
    }
    return graph;
}

fn main() {
    let mut graph = read_graph(stdin());
    for n in graph.nodes.iter() {
        println!("{}", n);
    }
}
