extern crate time;
extern crate rand;

mod cluster;
mod sort;
mod closest_pair;
mod clustering;

use std::fs::File;
use std::io::{BufReader, BufRead};
//use std::sync::Arc;
use cluster::Cluster;
use clustering::{herarchical_clustering, RunningMode};

fn main() {
    //let cluster_list = read_cluster_list("../unifiedCancerData_24.csv".to_owned());
    //let mut cluster_list = read_cluster_list("../unifiedCancerData_111.csv".to_owned());
    //let mut cluster_list = read_cluster_list("../unifiedCancerData_290.csv".to_owned());
    let mut cluster_list = read_cluster_list("../unifiedCancerData_896.csv".to_owned());
    //let mut cluster_list = read_cluster_list("../unifiedCancerData_3108.csv".to_owned());
    //let cluster_list: Vec<Cluster> = random_cluster_list_generator(10000);
    println!("read data length: {}", cluster_list.len());

    let test_data_1 = cluster_list.clone();
    let start_time = time::now();
    let result: Vec<Cluster> = 
        herarchical_clustering(test_data_1, 5, RunningMode::BruteForce).unwrap();
    let stop_time = time::now();
    println!("Time: {}", stop_time - start_time);
    //println!("{:?}", result);

    let test_data_2 = cluster_list.clone();
    let start_time = time::now();
    let result: Vec<Cluster> = 
        herarchical_clustering(test_data_2, 5, RunningMode::DivideConquer).unwrap();
    let stop_time = time::now();
    println!("Time: {}", stop_time - start_time);
    //println!("{:?}", result);

    let test_data_3 = cluster_list.clone();
    let start_time = time::now();
    let result: Vec<Cluster> = 
        herarchical_clustering(test_data_3, 5, RunningMode::ParalDivideConquer).unwrap();
    let stop_time = time::now();
    println!("Time: {}", stop_time - start_time);
    //println!("{:?}", result);

}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn random_cluster_list_generator(size: usize) -> Vec<Cluster> {
    let mut result: Vec<Cluster> = Vec::new();
    for _ in 0..size {
        let cluster = Cluster::new(rand::random::<u64>(),
                                   rand::random::<f64>(),
                                   rand::random::<f64>(),
                                   rand::random::<u32>() as u64,
                                   rand::random::<f64>());
        result.push(cluster);
    }
    result
}
