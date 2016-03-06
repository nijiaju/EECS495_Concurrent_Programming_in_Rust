use cluster::Cluster;
use std::sync::Arc;

// do not mutate the original vector, return a new sorted vector
// with indices in it.
pub fn merge_sort (cluster_list: Arc<Vec<Cluster>>, sort_by_x: bool) -> Vec<usize> {
    merge_sort_helper(cluster_list.clone(), sort_by_x, 0, cluster_list.len())
}

fn merge_sort_helper(cluster_list: Arc<Vec<Cluster>>,
                     sort_by_x: bool, start: usize, end: usize)
-> Vec<usize> {
    //base case
    if end - start == 1 {
        return vec![start];
    }

    let mid: usize = start + (end - start) / 2;
    let left = merge_sort_helper(cluster_list.clone(), sort_by_x, start, mid);
    let right = merge_sort_helper(cluster_list.clone(), sort_by_x, mid, end);

    let mut left_index: usize = 0;
    let mut right_index: usize = 0;
    let mut result: Vec<usize> = Vec::new();

    while left_index != left.len() && right_index != right.len() {
        if sort_by_x {
            if cluster_list[left[left_index]].horiz_center() < 
               cluster_list[right[right_index]].horiz_center() {
                result.push(left[left_index]);
                left_index += 1;
            } else {
                result.push(right[right_index]);
                right_index += 1;
            }
        } else {
            if cluster_list[left[left_index]].vert_center() <
               cluster_list[right[right_index]].vert_center() {
                result.push(left[left_index]);
                left_index += 1;
            } else {
                result.push(right[right_index]);
                right_index += 1;
            }
        }
    }

    while left_index != left.len() {
        result.push(left[left_index]);
        left_index += 1;
    }

    while right_index != right.len() {
        result.push(right[right_index]);
        right_index += 1;
    }

    result
}
