use cluster::Cluster;
use closest_pair::{bf_closest_pair,  serial_closest_pair, paral_closest_pair};
use std::sync::Arc;

pub enum RunningMode {
    BruteForce,
    DivideConquer,
    ParalDivideConquer,
}

pub fn herarchical_clustering(mut cluster_list: Vec<Cluster>, k: usize, t: RunningMode) 
    -> Option<Vec<Cluster>> {
    let mut cluster_list_arc: Arc<Vec<Cluster>> = Arc::new(cluster_list);

    while cluster_list_arc.len() > k {
        // println!("{}", cluster_list_arc.len());
        let min_distance = match t {
            RunningMode::BruteForce =>
                bf_closest_pair(cluster_list_arc.clone()).unwrap(),
            RunningMode::DivideConquer =>
                serial_closest_pair(cluster_list_arc.clone()).unwrap(),
            RunningMode::ParalDivideConquer =>
                paral_closest_pair(cluster_list_arc.clone()).unwrap(),
        };
        cluster_list = match Arc::try_unwrap(cluster_list_arc) {
            Ok(c)   => c,
            Err(_)  => return None,
        };
        // println!("{:?}", min_distance);
        let deleted_cluster: Cluster = cluster_list.remove(min_distance.2);
        cluster_list[min_distance.1].merge_clusters(deleted_cluster);        
        cluster_list_arc = Arc::new(cluster_list);
    }

    match Arc::try_unwrap(cluster_list_arc) {
        Ok(c)   => return Some(c),
        Err(_)  => return None,
    }
}
