extern crate time;

use cluster::Cluster;
use std::{cmp, f64};
use std::thread;

fn pair_distance(cluster_list: &Vec<Cluster>, index1: usize, index2: usize)
   -> (f64, usize, usize) {
    (cluster_list[index1].distance(&cluster_list[index2]),
     cmp::min(index1, index2), cmp::max(index1, index2))
}

pub fn bf_closest_pair(cluster_list: &Vec<Cluster>) -> Option<(f64, usize, usize)> {
    if cluster_list.len() < 2 {
        return None;
    }

    let mut min_distance = (f64::INFINITY, usize::max_value(), usize::max_value());

    for i in 0 .. cluster_list.len() {
        for j in i + 1 .. cluster_list.len() {
            let distance = pair_distance(cluster_list, i, j);
            if distance.0 < min_distance.0 {
                min_distance = distance;
            }
        }
    }

    Some(min_distance)
}

pub fn paral_closest_pair(cluster_list: &Vec<Cluster>)
    -> Option<(f64, usize, usize)> {
    closest_pair(cluster_list, true)
}

pub fn serial_closest_pair(cluster_list: &Vec<Cluster>)
    -> Option<(f64, usize, usize)> {
    closest_pair(cluster_list, false)
}

fn closest_pair(cluster_list: &Vec<Cluster>, in_parallel: bool)
    -> Option<(f64, usize, usize)> {

//    let start_time = time::now();
    let mut cluster_list_sorted_x = cluster_list.clone();
    cluster_list_sorted_x.sort_by(|a, b| a.cmp_x(b));
    let mut cluster_list_sorted_y = cluster_list.clone();
    cluster_list_sorted_y.sort_by(|a, b| a.cmp_y(b));
//    let end_time = time::now();
//    println!("sort_time_cost: {}", end_time - start_time);

    closest_pair_helper(&cluster_list_sorted_x, &cluster_list_sorted_y, in_parallel)
}

fn closest_pair_helper(cluster_list_x: &Vec<Cluster>,
                       cluster_list_y: &Vec<Cluster>,
                       in_parallel: bool)
    -> Option<(f64, usize, usize)> {

    // base case
    if cluster_list_x.len() <= 3 {
        return bf_closest_pair(cluster_list_x);
    }

    let m: usize = cluster_list_x.len() / 2;
    let mid: f64 = (cluster_list_x[m - 1].horiz_center() + 
                    cluster_list_x[m].horiz_center()) / 2 as f64;

    // prepare data for sub problems
    // TODO: eliminate the clone
    let mut left_h = Vec::new();
    let mut left_v = Vec::new();
    let mut right_h = Vec::new();
    let mut right_v = Vec::new();
    
    for c in cluster_list_x {
        if c.horiz_center() < mid {
            left_h.push(c.clone());
        } else {
            right_h.push(c.clone());
        }
    }
    for c in cluster_list_y {
        if c.horiz_center() < mid {
            left_v.push(c.clone());
        } else {
            right_v.push(c.clone());
        }
    }

    let left_distance;
    let right_distance;

    if in_parallel && right_h.len() > 256 {
        // spwan a thread to solve the sub-problem independently
        let right_handle = thread::spawn(move || {
            closest_pair_helper(&right_h, &right_v, in_parallel)
        });

        left_distance = closest_pair_helper(&left_h, &left_v, in_parallel).unwrap();
        right_distance = right_handle.join().unwrap().unwrap();
    } else {
        left_distance = closest_pair_helper(&left_h, &left_v, in_parallel).unwrap();
        right_distance = closest_pair_helper(&right_h, &right_v, in_parallel).unwrap();
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
    for c in cluster_list_y {
        if (c.horiz_center() - mid).abs() < min_distance.0 {
            s.push(c.clone());
        }
    }

    // find the minmum distance
    let k = s.len();
    if k > 2 {
        for u in 0 .. k - 2 {
            for v in u + 1 .. cmp::min(u + 3, k - 1) {
                let distance = s[u].distance(&s[v]);
                if distance < min_distance.0 {
                    min_distance = (distance, 0, 0);
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
