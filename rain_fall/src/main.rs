fn main() {
	let measurements = read_measurements(stdin());
	product_output(&calculate_results(&measurements));
}

struct Results {
	mean  f64,
	above usize,
	below usize,
}

fn read_measurements<R: Read>(reader: R) -> Vec<f64> {
	let mut measurements = vec![]; // Vec::new()

