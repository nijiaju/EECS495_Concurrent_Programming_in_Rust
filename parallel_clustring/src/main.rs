extern crate time;
extern crate rand;

mod cluster;
mod sort;
mod closest_pair;
mod clustering;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::Arc;
use cluster::Cluster;

fn main() {
    //let mut cluster_list = read_cluster_list("../unifiedCancerData_24.csv".to_owned());
    //let mut cluster_list = read_cluster_list("../unifiedCancerData_111.csv".to_owned());
    //let mut cluster_list = read_cluster_list("../unifiedCancerData_290.csv".to_owned());
    //let mut cluster_list = read_cluster_list("../unifiedCancerData_896.csv".to_owned());
    //let mut cluster_list = read_cluster_list("../unifiedCancerData_3108.csv".to_owned());
    let mut cluster_list = random_cluster_list_generator(10000);
    println!("read data length: {}", cluster_list.len());
    //let data = Arc::new(cluster_list);
    {
        let start_time = time::now();
        let min_distance1 = closest_pair::bf_closest_pair(&cluster_list);
        let stop_time = time::now();
        println!("min distance1: {:?}", min_distance1);
        println!("{}", stop_time - start_time);
    }
    {
        let start_time = time::now();
        let min_distance2 = closest_pair::serial_closest_pair(&cluster_list);
        let stop_time = time::now();
        println!("min distance2: {:?}", min_distance2);
        println!("{}", stop_time - start_time);
    }
    {
        let start_time = time::now();
        let min_distance3 = closest_pair::paral_closest_pair(&cluster_list);
        let stop_time = time::now();
        println!("min distance2: {:?}", min_distance3);
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
    result
}

fn random_cluster_list_generator(size: usize) -> Vec<Cluster> {
    let mut result: Vec<Cluster> = Vec::new();
    for _ in 0..size {
        let cluster = Cluster::new(rand::random::<u64>(),
                                   rand::random::<f64>(),
                                   rand::random::<f64>(),
                                   rand::random::<u64>(),
                                   rand::random::<f64>());
        result.push(cluster);
    }
    result
}
