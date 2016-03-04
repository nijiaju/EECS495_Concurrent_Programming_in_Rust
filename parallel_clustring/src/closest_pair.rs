use cluster::Cluster;
use std::{cmp, f64};

fn pair_distance(cluster_list: &Vec<Cluster>, index1: usize, index2: usize)
   -> (f64, usize, usize) {
    (cluster_list[index1].distance(&cluster_list[index2]),
     cmp::min(index1, index2), cmp::max(index1, index2))
}

fn bf_closest_pair(cluster_list: &Vec<Cluster>) -> (f64, usize, usize) {
    let mut min_distance = (f64::INFINITY, usize::max_value(), usize::max_value());

    for i in 0 .. cluster_list.len() {
        for j in i + 1 .. cluster_list.len() {
            let distance = pair_distance(cluster_list, i, j);
            if distance.0 < min_distance.0 {
                min_distance = distance;
            }
        }
    }

    min_distance
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
        let result: Vec<Cluster> = Vec::new();

        while let Some(Ok(line)) = lines.next() {
            let mut split = line.split(", ");
            println!("{}", split.next().unwrap());
        }
        return result;
    }

    #[test]
    fn bf_closeset_pair_1() {
        read_cluster_list("unifiedCancerData_24.csv".to_owned());
    }
}
