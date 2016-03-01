"""
Student template code for Project 3
Student will implement five functions:

slow_closest_pair(cluster_list)
fast_closest_pair(cluster_list)
closest_pair_strip(cluster_list, horiz_center, half_width)
hierarchical_clustering(cluster_list, num_clusters)
kmeans_clustering(cluster_list, num_clusters, num_iterations)

where cluster_list is a 2D list of clusters in the plane
"""

import math
import alg_cluster



######################################################
# Code for closest pairs of clusters

def pair_distance(cluster_list, idx1, idx2):
    """
    Helper function that computes Euclidean distance between two clusters in a list

    Input: cluster_list is list of clusters, idx1 and idx2 are integer indices for two clusters
    
    Output: tuple (dist, idx1, idx2) where dist is distance between
    cluster_list[idx1] and cluster_list[idx2]
    """
    return (cluster_list[idx1].distance(cluster_list[idx2]), min(idx1, idx2), max(idx1, idx2))


def slow_closest_pair(cluster_list):
    """
    Compute the distance between the closest pair of clusters in a list (slow)

    Input: cluster_list is the list of clusters
    
    Output: tuple of the form (dist, idx1, idx2) where the centers of the clusters
    cluster_list[idx1] and cluster_list[idx2] have minimum distance dist.       
    """
    
    min_distance = (float('inf'), -1, -1)
    
    for index1 in range(len(cluster_list)):
        for index2 in range(index1 + 1, len(cluster_list)):
            if index1 == index2:
                print "Error: Index1 should never equal to Index2"
            distance = cluster_list[index1].distance(cluster_list[index2])
            if distance < min_distance[0]:
                min_distance = (distance, index1, index2)
    
    return min_distance


def fast_closest_pair(cluster_list):
    """
    Compute the distance between the closest pair of clusters in a list (fast)

    Input: cluster_list is list of clusters SORTED such that horizontal positions of their
    centers are in ascending order
    
    Output: tuple of the form (dist, idx1, idx2) where the centers of the clusters
    cluster_list[idx1] and cluster_list[idx2] have minimum distance dist.       
    """
#    cluster_list.sort(key = lambda cluster: cluster.horiz_center())
#    indices = [idx for idx in xrange(len(cluster_list))]
#    indices.sort(key = lambda idx: cluster_list[idx].horiz_center())

    length = len(cluster_list)
    if length <= 3:
        return slow_closest_pair(cluster_list)

    middle = length / 2
    first_half = cluster_list[:middle]
    second_half = cluster_list[middle:]

    first_half_distance = fast_closest_pair(first_half)
    second_half_distance = fast_closest_pair(second_half)

    if first_half_distance[0] < second_half_distance[0]:
        min_distance = first_half_distance
    else:
        min_distance = (second_half_distance[0], second_half_distance[1] + middle, second_half_distance[2] + middle)

    mid = 0.5 * (cluster_list[middle - 1].horiz_center() + cluster_list[middle].horiz_center())
    min_strip_distance = closest_pair_strip(cluster_list, mid, min_distance[0])

    if min_distance[0] < min_strip_distance[0]:
        return min_distance
    else:
        return min_strip_distance


def closest_pair_strip(cluster_list, horiz_center, half_width):
    """
    Helper function to compute the closest pair of clusters in a vertical strip
    
    Input: cluster_list is a list of clusters produced by fast_closest_pair
    horiz_center is the horizontal position of the strip's vertical center line
    half_width is the half the width of the strip (i.e; the maximum horizontal distance
    that a cluster can lie from the center line)

    Output: tuple of the form (dist, idx1, idx2) where the centers of the clusters
    cluster_list[idx1] and cluster_list[idx2] lie in the strip and have minimum distance dist.       
    """

    indices = [idx for idx in xrange(len(cluster_list))]
    indices.sort(key = lambda idx: cluster_list[idx].vert_center())
    strip_list = []
    strip_list_indices = []
    for index in xrange(len(cluster_list)):
        if abs(horiz_center - cluster_list[indices[index]].horiz_center()) < half_width:
            strip_list.append(cluster_list[indices[index]])
            strip_list_indices.append(indices[index])
    
    length = len(strip_list)
    min_distance = (float('inf'), -1, -1)

    for dummy_u in range(length - 1):
        dummy_v = dummy_u + 1
        while dummy_v <= dummy_u + 3 and dummy_v < length:
            distance = (strip_list[dummy_u].distance(strip_list[dummy_v]), 
                        min(strip_list_indices[dummy_u], strip_list_indices[dummy_v]),
                        max(strip_list_indices[dummy_u], strip_list_indices[dummy_v]))
            if distance[0] < min_distance[0]:
                min_distance = distance
            dummy_v += 1
    return min_distance
            
 
    
######################################################################
# Code for hierarchical clustering


def hierarchical_clustering(cluster_list, num_clusters):
    """
    Compute a hierarchical clustering of a set of clusters
    Note: the function may mutate cluster_list
    
    Input: List of clusters, integer number of clusters
    Output: List of clusters whose length is num_clusters
    """

    length = len(cluster_list)
    cluster_list.sort(key = lambda cluster: cluster.horiz_center())

    while length > num_clusters:
        min_distance = fast_closest_pair(cluster_list)
        cluster_list[min_distance[1]].merge_clusters(cluster_list[min_distance[2]])
        cluster_list.pop(min_distance[2])
        cluster_list.sort(key = lambda cluster: cluster.horiz_center())
        length -= 1

    return cluster_list


######################################################################
# Code for k-means clustering

    
def kmeans_clustering(cluster_list, num_clusters, num_iterations):
    """
    Compute the k-means clustering of a set of clusters
    Note: the function may not mutate cluster_list
    
    Input: List of clusters, integers number of clusters and number of iterations
    Output: List of clusters whose length is num_clusters
    """

    cpoied_cluster_list = cluster_list[:]
    cpoied_cluster_list.sort(key = lambda cluster: cluster.total_population())
    centers = [cpoied_cluster_list[index] for index in range(-1, -num_clusters - 1, -1)]

    for dummy_iteration in range(num_iterations):
        cluster_sets = [alg_cluster.Cluster(set([]), 0, 0, 0, 0) for dummy_j in range(num_clusters)]
        for each_cluster in range(len(cluster_list)):
            min_distance = (float('inf'), -1)
            for each_center in range(num_clusters):
                distance = (cluster_list[each_cluster].distance(centers[each_center]), each_center)
                if distance[0] < min_distance[0]:
                    min_distance = distance
            cluster_sets[min_distance[1]].merge_clusters(cluster_list[each_cluster])
        centers = cluster_sets
            
    return cluster_sets