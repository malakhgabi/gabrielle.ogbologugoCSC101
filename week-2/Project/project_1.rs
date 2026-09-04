fn main() {
	let p: f64 = 520_000_000.0;
	let r: f64 = 10.0;
	let n: f64 = 5.0;

	//Compound Interest
	let a: f64 = p * ( 1.0 + ( r / 100.0)).powf(n);
	let ci: f64 = a - p;
	println! ("Total amount is {}", a);
	println! ("Compound Interest is {}", ci);
}
