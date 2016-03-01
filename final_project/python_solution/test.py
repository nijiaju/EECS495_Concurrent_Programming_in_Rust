import project3
import alg_cluster

DATA111_PATH = "/Users/nijiaju/OneDrive/MOOC/Algorithmic Thinking II/Module3/Project/unifiedCancerData_111.csv"
DATA3108_PATH = "/Users/nijiaju/OneDrive/MOOC/Algorithmic Thinking II/Module3/Project/unifiedCancerData_3108.csv"
DATA24_PATH = "/Users/nijiaju/OneDrive/MOOC/Algorithmic Thinking II/Module3/Project/unifiedCancerData_24.csv"

def load_data(path, amount = 0):
    """
    Function that loads county-level data about lifetime cancer risk from air toxics
    and constructs a cluster list base on the data loaded.
    Amount stands for the lines that should be loaded. 0 means load the whole data sheet.

    Return a cluster list.
    """

    cluster_list = []
    load_all = True
    data_sheet = open(path, 'r')

    if amount > 0:
        load_all = False

    while True:
        line = data_sheet.readline()
        line = line.strip()
        if len(line) == 0:
            break

        line_list = line.split(', ')
        cluster = alg_cluster.Cluster(set([line_list[0]]), float(line_list[1]), float(line_list[2]), int(line_list[3]), float(line_list[4]))
        cluster_list.append(cluster)

        if load_all == False:
            amount -= 1
            if amount == 0:
                break

    data_sheet.close()
    return cluster_list


cluster_list = load_data(DATA24_PATH)
#print project3.hierarchical_clustering(cluster_list, 5)
print project3.kmeans_clustering(cluster_list, 15, 1)
#print project3.slow_closest_pair(cluster_list)
#print project3.fast_closest_pair(cluster_list)