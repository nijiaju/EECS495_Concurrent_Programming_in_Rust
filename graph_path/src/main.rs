/// EECS495 Concurrent Programming in Rust
/// ======================================
/// Homework3 Graph
/// ---------------
///
/// Prints the path between two nodes given in the user queries.
///
/// # Assumptions
/// 1. The graph given in the graph specification file will be treated as undirected graph. 
/// 2. The graph specification file may be incomplete. The edge between node A and node B may be
/// appear only once in the file.
/// 3. If there are more than one path between the nodes, choose the shortest one.
/// 4. The user query should follow the format `node1 node2`, else this program may panic.


use std::io::{stdin, Read, BufReader, BufRead};
use std::fs::File;
use std::env;

type NameTable  = std::collections::HashMap<String, usize>;

struct Node {
    name:       String,
    neighbours: Vec<usize>,
}

struct Graph {
    nodes:      Vec<Node>,
    name_table: NameTable,
}

struct NodeData {
    visited:    bool,
    distance:    usize,
    parent:     usize,
}

impl Node {
    fn new(name: String) -> Self {
        Node {
            name:       name,
            neighbours: Vec::new()
        }
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

impl NodeData {
    fn new() -> Self {
        NodeData {
            visited:    false,
            distance:   usize::max_value(),
            parent:    usize::max_value(),
        }
    }
}

impl Clone for NodeData {
    fn clone(&self) -> Self {
        NodeData {
            visited:    self.visited,
            distance:   self.distance,
            parent:     self.parent,
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

fn search_graph_path(graph: &Graph, from_node: &str, to_node: &str) -> Result<String, &'static str> {
        type Queue = std::collections::LinkedList<usize>;
        let mut fifo = Queue::new();
        let mut nodes_data = Vec::new();
        nodes_data.resize(graph.nodes.len(), NodeData::new());
        let from_node_id =  match graph.name_table.get(from_node) {
                                Some(&n) => n,
                                None     => return Err("Invalid Node Name!"),
                            };
        fifo.push_back(from_node_id);
        nodes_data[from_node_id].distance = 0usize;

        // Do breadth first search
        while !fifo.is_empty() {
            let current_node = fifo.pop_front().unwrap();
            nodes_data[current_node].visited = true;
            for &neighbour in &graph.nodes[current_node].neighbours {
                if nodes_data[neighbour].visited {
                    continue;
                } else {
                    if nodes_data[neighbour].distance >
                        nodes_data[current_node].distance + 1 {
                        nodes_data[neighbour].parent = current_node;
                        nodes_data[neighbour].distance = 
                            nodes_data[current_node].distance + 1;
                    }
                    fifo.push_back(neighbour);
                }
            }
        }

        // Trace back from the destnation node
        let mut node_id  =  match graph.name_table.get(to_node) {
                                Some(&n) => n,
                                None     => return Err("Invalid Node Name!"),
                            };
        if nodes_data[node_id].parent == usize::max_value() {
            return Err("No path exist!");
        }
        let mut path = Vec::new();
        while node_id != from_node_id {
            path.push(node_id);
            node_id = nodes_data[node_id].parent;
        }
        path.push(node_id);

        // Print the result
        let mut output = String::new();
        while let Some(n) = path.pop() {
            output.push_str(&graph.nodes[n].name);
            output.push(' ');
        }
        return Ok(output);
}

fn main() {
    //argument sanity check
    if env::args().count() != 2usize {
        println!("\nuseage: cargo run graph.dat\n");
        return;
    }

    //open the graph description file
    let f = match File::open(env::args().nth(1).unwrap()) {
        Ok(file) => file,
        Err(e)   => {
            println!("{}", e);
            return;
        }
    };

    //Search the shortest path
    let graph  = read_graph(f);
    let mut lines = BufReader::new(stdin()).lines();
    while let Some(Ok(line)) = lines.next() {
        let nodes: Vec<&str>  = line.split(" ").collect();
        if nodes.len() != 2 {
            println!("Invalid Query!");
            continue;
        }

        let result = search_graph_path(&graph, nodes[0], nodes[1]);
        match result {
            Err(e) => println!("{}", e),
            Ok(r)  => println!("{}", r),
        };
    }
}

#[cfg(test)]
mod graph_path_test {
    use super::{read_graph, search_graph_path};
    use std::fs::File;

    #[test]
    fn query_empty_graph() {
        let f = File::open("graph_files/empty_graph.dat").unwrap();
        let graph = read_graph(f);
        assert_eq!(search_graph_path(&graph, "a", "b"),
                   Err("Invalid Node Name!"));
    }

    #[test]
    fn query_1v1e_graph() {
        let f = File::open("graph_files/1v1e_graph.dat").unwrap();
        let graph = read_graph(f);
        assert_eq!(search_graph_path(&graph, "a", "b"),
                   Ok("a b ".to_owned()));
        assert_eq!(search_graph_path(&graph, "b", "a"),
                   Ok("b a ".to_owned()));
        assert_eq!(search_graph_path(&graph, "c", "a"),
                   Err("Invalid Node Name!"));
    }

    #[test]
    fn query_4v4e_graph() {
        let f = File::open("graph_files/4v4e_graph.dat").unwrap();
        let graph = read_graph(f);
        assert_eq!(search_graph_path(&graph, "a", "d"),
                   Ok("a c d ".to_owned()));
        assert_eq!(search_graph_path(&graph, "b", "c"),
                   Ok("b c ".to_owned()));
    }

    #[test]
    fn query_10v15e_graph() {
        let f = File::open("graph_files/10v15e_graph.dat").unwrap();
        let graph = read_graph(f);
        assert_eq!(search_graph_path(&graph, "a", "b"),
                   Ok("a e b ".to_owned()));
        assert_eq!(search_graph_path(&graph, "g", "a"),
                   Ok("g c d h a ".to_owned()));
        assert_eq!(search_graph_path(&graph, "f", "j"),
                   Ok("f d j ".to_owned()));
    }

    #[test]
    fn query_8v6e_unconnected_graph() {
        let f = File::open("graph_files/8v6e_unconnected_graph.dat").unwrap();
        let graph = read_graph(f);
        assert_eq!(search_graph_path(&graph, "e", "h"),
                   Ok("e g f h ".to_owned()));
        assert_eq!(search_graph_path(&graph, "a", "e"),
                   Err("No path exist!"));
    }
}
