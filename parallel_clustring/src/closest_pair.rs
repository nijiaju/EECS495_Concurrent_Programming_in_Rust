extern crate time;

use cluster::Cluster;
use sort::merge_sort;
use std::{cmp, f64};
use std::thread;
use std::sync::Arc;

fn pair_distance(cluster_list: Arc<Vec<Cluster>>, index1: usize, index2: usize)
   -> (f64, usize, usize) {
    (cluster_list[index1].distance(&cluster_list[index2]),
     cmp::min(index1, index2), cmp::max(index1, index2))
}

pub fn bf_closest_pair(cluster_list: Arc<Vec<Cluster>>) -> Option<(f64, usize, usize)> {
    if cluster_list.len() < 2 {
        return None;
    }

    let mut min_distance = (f64::INFINITY, usize::max_value(), usize::max_value());

    for i in 0 .. cluster_list.clone().len() {
        for j in i + 1 .. cluster_list.clone().len() {
            let distance = pair_distance(cluster_list.clone(), i, j);
            if distance.0 < min_distance.0 {
                min_distance = distance;
            }
        }
    }

    Some(min_distance)
}

pub fn paral_closest_pair(cluster_list: Arc<Vec<Cluster>>)
    -> Option<(f64, usize, usize)> {
    closest_pair(cluster_list, true)
}

pub fn serial_closest_pair(cluster_list: Arc<Vec<Cluster>>)
    -> Option<(f64, usize, usize)> {
    closest_pair(cluster_list, false)
}

fn closest_pair(cluster_list: Arc<Vec<Cluster>>, in_parallel: bool)
    -> Option<(f64, usize, usize)> {

//    let start_time = time::now();
//    let mut cluster_list_sorted_x = cluster_list.clone();
//    cluster_list_sorted_x.sort_by(|a, b| a.cmp_x(b));
//    let mut cluster_list_sorted_y = cluster_list.clone();
//    cluster_list_sorted_y.sort_by(|a, b| a.cmp_y(b));
//    let end_time = time::now();
//    println!("sort_time_cost: {}", end_time - start_time);
    
    let start_time = time::now();
    let cluster_list_index_h = merge_sort(cluster_list.clone(), true, in_parallel);
    let cluster_list_index_v = merge_sort(cluster_list.clone(), false, in_parallel);
    let end_time = time::now();
    println!("sort_time_cost: {}", end_time - start_time);
    
    closest_pair_helper(cluster_list,
                        cluster_list_index_h,
                        cluster_list_index_v,
                        in_parallel)
}

fn closest_pair_helper(cluster_list: Arc<Vec<Cluster>>,
                       cluster_list_index_x: Vec<usize>,
                       cluster_list_index_y: Vec<usize>,
                       in_parallel: bool)
    -> Option<(f64, usize, usize)> {

    // base case
    if cluster_list_index_x.len() <= 3 {
        let mut cluster_list_: Vec<Cluster> = Vec::new();
        for &i in &cluster_list_index_x {
            cluster_list_.push(cluster_list[i].clone());
        }
        return bf_closest_pair(Arc::new(cluster_list_));
    }

    let m: usize = cluster_list_index_x.len() / 2;
    let mid: f64 = (cluster_list[cluster_list_index_x[m - 1]].horiz_center() + 
                    cluster_list[cluster_list_index_x[m]].horiz_center()) / 2 as f64;

    // prepare data for sub problems
    let mut left_index_h = Vec::new();
    let mut left_index_v = Vec::new();
    let mut right_index_h = Vec::new();
    let mut right_index_v = Vec::new();
    
    for &i in &cluster_list_index_x {
        if cluster_list[i].horiz_center() < mid {
            left_index_h.push(i);
        } else {
            right_index_h.push(i);
        }
    }
    for &i in &cluster_list_index_y {
        if cluster_list[i].horiz_center() < mid {
            left_index_v.push(i);
        } else {
            right_index_v.push(i);
        }
    }

    let left_distance;
    let right_distance;

    let cluster_list_r = cluster_list.clone();
    let cluster_list_l = cluster_list.clone();
    if in_parallel && right_index_h.len() > 256 {
        // spwan a thread to solve the sub-problem independently
        let right_handle = thread::spawn(move || {
            closest_pair_helper(cluster_list_r, right_index_h, right_index_v, in_parallel)
        });

        left_distance = closest_pair_helper(cluster_list_l, left_index_h, left_index_v, in_parallel).unwrap();
        right_distance = right_handle.join().unwrap().unwrap();
    } else {
        left_distance = closest_pair_helper(cluster_list_l, left_index_h, left_index_v, in_parallel).unwrap();
        right_distance = closest_pair_helper(cluster_list_r, right_index_h, right_index_v, in_parallel).unwrap();
    }

    // find the min distance from left and right
    let mut min_distance;
    if left_distance.0 < right_distance.0 {
        min_distance = left_distance;
    } else {
        min_distance = right_distance;
    }

    // find the points in the strip
    let mut s = Vec::new();
    for &i in &cluster_list_index_y {
        if (cluster_list[i].horiz_center() - mid).abs() < min_distance.0 {
            s.push(i);
        }
    }

    // find the minmum distance
    let k = s.len();
    if k > 2 {
        for u in 0 .. k - 2 {
            for v in u + 1 .. cmp::min(u + 3, k - 1) {
                let distance = cluster_list[s[u]].distance(&cluster_list[s[v]]);
                if distance < min_distance.0 {
                    min_distance = (distance, s[u], s[v]);
                }
            }
         }
    }

    Some(min_distance)
}

#[cfg(test)]
mod closest_pair_test {
    use super::bf_closest_pair;
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use cluster::Cluster;

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

    #[test]
    fn bf_closeset_pair_1() {
        let mut cluster_list = read_cluster_list("unifiedCancerData_24.csv".to_owned());
        let min_distance = bf_closest_pair(cluster_list);
    }
}

