mod cluster;
mod closest_pair;
mod clustering;

use std::fs::File;
use std::io::{BufReader, BufRead};
use cluster::Cluster;

fn main() {
    //let mut cluster_list = read_cluster_list("../unifiedCancerData_24.csv".to_owned());
    let mut cluster_list = read_cluster_list("../unifiedCancerData_3108.csv".to_owned());
    println!("read data length: {}", cluster_list.len());
    let min_distance = closest_pair::bf_closest_pair(&cluster_list);
    println!("min distance: {:?}", min_distance);
}

fn read_cluster_list(file_name: String) -> Vec<Cluster> {
    let f = File::open(file_name).unwrap(); 
    let mut lines = BufReader::new(f).lines();
    let mut result: Vec<Cluster> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        let mut split = line.split(", ");
        let cluster = Cluster::new(split.next().unwrap().parse::<u64>().unwrap(),
                                   split.next().unwrap().parse::<f64>().unwrap(),
                                   split.next().unwrap().parse::<f64>().unwrap(),
                                   split.next().unwrap().parse::<u64>().unwrap(),
                                   split.next().unwrap().parse::<f64>().unwrap());
        result.push(cluster);
    }
    return result;
}
