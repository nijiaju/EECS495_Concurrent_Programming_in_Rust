use std::io::{stdin, Read, BufReader, BufRead};
use std::fmt;
use std::fs::File;
use std::env;

type NameTable  = std::collections::HashMap<String, usize>;

struct Node {
    name:       String,
    visited:    bool,
    distance:   usize,
    parent:     usize,
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
            visited:    false,
            distance:   usize::max_value(),
            parent:     usize::max_value(),
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
                if !graph.nodes[node].neighbours.contains(&n) {
                    graph.nodes[node].neighbours.push(n);
                }
                if !graph.nodes[n].neighbours.contains(&node) {
                    graph.nodes[n].neighbours.push(node);
                }
            }
        }
    }
    return graph;
}

fn main() {
    //argument sanity check
    if env::args().count() != 2usize {
        println!("\nuseage: cargo run graph.dat\n");
    }

    //open the graph description file
    let f = match File::open(env::args().nth(1).unwrap()) {
        Ok(file) => file,
        Err(e)   => {
            println!("{}", e);
            return;
        }
    };

    //read the graph description file and build the graph data structure
    let mut graph = read_graph(f);
    for n in graph.nodes.iter() {
        println!("{}", n);
    }

    let mut lines = BufReader::new(stdin()).lines();
    while let Some(Ok(line)) = lines.next() {
        let mut nodes = line.split(" ");
        let from_node = nodes.next().unwrap();
        let to_node   = nodes.next().unwrap();
        println!("from {} to {}", from_node, to_node);
       
        type Queue = std::collections::LinkedList<usize>;
        let mut fifo = Queue::new();
        fifo.push_back(match graph.name_table.get(from_node) {
                            Some(n) => *n,
                            None    => {
                                println!("Invalid Node Name!");
                                return;
                            }
                       });
        while !fifo.is_empty() {
            let current_node = fifo.pop_front().unwrap();
            graph.nodes[current_node].visited = true;
            for neighbour in &graph.nodes[current_node].neighbours {
                if !graph.nodes[*neighbour].visited {
                    continue;
                } else {
                    graph.nodes[*neighbour].visited = true;
                    graph.nodes[*neighbour].parent = current_node;
                    graph.nodes[*neighbour].distance = 
                        graph.nodes[current_node].distance + 1;
                    fifo.push_back(*neighbour);
                }
            }
        }
    }
}
