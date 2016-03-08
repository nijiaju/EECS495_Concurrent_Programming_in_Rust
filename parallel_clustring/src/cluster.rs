use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Cluster {
    fips_codes:         Vec<u64>,
    horiz_center:       f64,
    vert_center:        f64,
    total_population:   u64,
    averaged_risk:      f64,
}

impl Cluster {
    pub fn new(fips: u64, horiz: f64, vert: f64, population: u64, risk: f64)
           -> Self {
               Cluster {
                   fips_codes:          vec![fips],
                   horiz_center:        horiz,
                   vert_center:         vert,
                   total_population:    population,
                   averaged_risk:       risk,
                }
           }

    #[allow(dead_code)]
    pub fn fips_codes(&self) -> &Vec<u64> {
        &self.fips_codes
    }

    pub fn horiz_center(&self) -> f64 {
        self.horiz_center
    }

    pub fn vert_center(&self) -> f64 {
        self.vert_center
    }

    pub fn total_population(&self) -> u64 {
        self.total_population
    }

    pub fn averaged_risk(&self) -> f64 {
        self.averaged_risk
    }

    // temporary allow dead code
    // remove this after the parallal sort function takes a lambda expression
    #[allow(dead_code)]
    pub fn cmp_x(&self, other_cluster: &Self) -> Ordering {

        if self.horiz_center < other_cluster.horiz_center() {
            Ordering::Less
        } else if self.horiz_center == other_cluster.horiz_center() {
            Ordering::Equal
        } else {
            Ordering::Greater
        }

        // f64 do not have trait Ord
        //self.horiz_center.cmp(other_cluster.horiz_center())
    }

    // temporary allow dead code
    // remove this after the parallal sort function takes a lambda expression
    #[allow(dead_code)]
    pub fn cmp_y(&self, other_cluster: &Self) -> Ordering {
        if self.vert_center < other_cluster.vert_center() {
            Ordering::Less
        } else if self.vert_center == other_cluster.vert_center() {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }

    pub fn distance(&self, other_cluster: &Cluster) -> f64 {
        // Compute the Euclidean distance between two clusters
        let vert_dist = self.vert_center - other_cluster.vert_center;
        let horiz_dist = self.horiz_center - other_cluster.horiz_center;
        (vert_dist * vert_dist + horiz_dist * horiz_dist).sqrt()
    }
    
    // take the owenership of the other_cluster
    pub fn merge_clusters(&mut self, other_cluster: Cluster) {
        if other_cluster.fips_codes.len() == 0 {
            return;
        } else {
            self.fips_codes.append(&mut other_cluster.fips_codes.clone());
            let mut self_weight = self.total_population as f64;
            let mut other_weight = other_cluster.total_population() as f64;
            self.total_population += other_cluster.total_population();
            self_weight /= self.total_population as f64;
            other_weight /= self.total_population as f64;

            self.vert_center = self_weight * self.vert_center
                               + other_weight * other_cluster.vert_center();
            self.horiz_center = self_weight * self.horiz_center
                                + other_weight * other_cluster.horiz_center();
            self.averaged_risk = self_weight * self.averaged_risk
                                 + other_weight * other_cluster.averaged_risk();
        }
    }

}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut codes = String::new();
        for code in &self.fips_codes {
            codes.push_str(&code.to_string());
        }
        write!(f, "cluster: {}\ncenter: ({}, {})\n",
               codes, self.horiz_center, self.vert_center)
    }
}
