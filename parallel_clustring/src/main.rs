mod cluster;

use cluster::Cluster;

fn main() {
    println!("Hello, world!");
    let c1 = Cluster::new(0, 3.0, 8.8, 0, 0.0);
    let c2 = Cluster::new(0, 4.0, 6.6, 0, 0.0);
    println!("{}", c1);
    println!("{:?}", c2);
    println!("{}", c1.distance(&c2));
}


// read data


// do clustring
// will call closest pair algo here
    // cloest pair will call sort here

// draw result
