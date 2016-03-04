extern crate time;

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
    {
        let start_time = time::now();
        let min_distance1 = closest_pair::bf_closest_pair(&cluster_list);
        let stop_time = time::now();
        println!("min distance1: {:?}", min_distance1);
        println!("{}", stop_time - start_time);
    }
    {
        let start_time = time::now();
        let min_distance2 = closest_pair::paral_closest_pair(&mut cluster_list);
        let stop_time = time::now();
        println!("min distance2: {:?}", min_distance2);
        println!("{}", stop_time - start_time);
    }
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
