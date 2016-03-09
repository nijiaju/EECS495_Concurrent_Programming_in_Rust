"""
Example code for creating and visualizing
cluster of county-based cancer risk data

Note that you must download the file
http://www.codeskulptor.org/#alg_clusters_matplotlib.py
to use the matplotlib version of this code
"""

import math
import random
import urllib2
import alg_cluster
import alg_clusters_matplotlib

###################################################
# Code to load data tables

# URLs for cancer risk data tables of various sizes
# Numbers indicate number of counties in data table

DIRECTORY = "/Users/jiajuni/Documents/Git/EECS495_Rust/parallel_clustring/src/"
DATA_3108_PATH = DIRECTORY + "unifiedCancerData_3108.csv"
DATA_896_PATH = DIRECTORY + "unifiedCancerData_896.csv"
DATA_290_PATH = DIRECTORY + "unifiedCancerData_290.csv"
DATA_111_PATH = DIRECTORY + "unifiedCancerData_111.csv"
DATA_24_PATH = DIRECTORY + "unifiedCancerData_24.csv"

def load_data_table(path):
    """
    Import a table of county-based cancer risk data
    from a csv format file
    """
    data_file = open(path, 'r')
    data = data_file.read()
    data_lines = data.split('\n')
    #print "Loaded", len(data_lines), "data points"
    data_tokens = [line.split(',') for line in data_lines]
    return [[tokens[0], float(tokens[1]), float(tokens[2]), int(tokens[3]), float(tokens[4])] 
            for tokens in data_tokens]

###################################################
# Code to load results

FILEPATH = "/Users/jiajuni/Documents/Git/EECS495_Rust/parallel_clustring/src/result"

def load_result(path):
    """
    Import a table of county-based cancer risk data
    from a csv format file
    """
    result = [];
    data_file = open(path, 'r')
    data = data_file.read()
    data_lines = data.split('\n')

    cluster_list = list()
    for i in range(0, len(data_lines) - 1, 2):
        fips = set(data_lines[i].split(' '))
        data = data_lines[i + 1].split(' ')
        cluster_list.append(alg_cluster.Cluster(fips, float(data[0]), float(data[1]), int(data[2]), float(data[3])))
    print "Loaded", len(cluster_list), "result points"

    return cluster_list


############################################################
# Code to create sequential clustering
# Create alphabetical clusters for county data

def sequential_clustering(singleton_list, num_clusters):
    """
    Take a data table and create a list of clusters
    by partitioning the table into clusters based on its ordering
    
    Note that method may return num_clusters or num_clusters + 1 final clusters
    """
    
    cluster_list = []
    cluster_idx = 0
    total_clusters = len(singleton_list)
    cluster_size = float(total_clusters)  / num_clusters
    
    for cluster_idx in range(len(singleton_list)):
        new_cluster = singleton_list[cluster_idx]
        if math.floor(cluster_idx / cluster_size) != \
           math.floor((cluster_idx - 1) / cluster_size):
            cluster_list.append(new_cluster)
        else:
            cluster_list[-1] = cluster_list[-1].merge_clusters(new_cluster)
            
    return cluster_list
                

#####################################################################
# Code to load cancer data, compute a clustering and 
# visualize the results


def run_example():
    """
    Load a data table, compute a list of clusters and 
    plot a list of clusters

    Set DESKTOP = True/False to use either matplotlib or simplegui
    """
    data_table = load_data_table(DATA_3108_PATH)
    cluster_list = load_result(FILEPATH)
    
    #for line in data_table:
    #    singleton_list.append(alg_cluster.Cluster(set([line[0]]), line[1], line[2], line[3], line[4]))
        
    #cluster_list = sequential_clustering(singleton_list, 15)	
    #print "Displaying", len(cluster_list), "sequential clusters"

    #cluster_list = project3.hierarchical_clustering(singleton_list, 9)
    print "Displaying", len(cluster_list), "hierarchical clusters"

    #cluster_list = alg_project3_solution.kmeans_clustering(singleton_list, 9, 5)	
    #print "Displaying", len(cluster_list), "k-means clusters"

    # draw the clusters using matplotlib or simplegui
    #alg_clusters_matplotlib.plot_clusters(data_table, cluster_list, False)
    alg_clusters_matplotlib.plot_clusters(data_table, cluster_list, True)  #add cluster centers
    
run_example()





    





  
        






        





