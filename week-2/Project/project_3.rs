// this project is going to find the depreciated price of a television

fn main() {
	let p: f64 = 210_000.0;
	let r: f64 = 5.0;
	let n: f64 = 3.0;

	let depreciation: f64 = p * (1.0 - r / 100.0.powf(n);
	println!("The depreciation value is {}", depreciation);

	let after: f64 = p - depreciation;

	println!("The price after the depreciation is {}", after_price);
}