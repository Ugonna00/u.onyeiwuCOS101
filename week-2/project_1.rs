fn main() {
	let r:f64 = 10.0;
	let n:f64 = 5.0;
	let p:f64 = 520000000.00;

	let a = p * (1.0 + (r / 100.0)).powf(n);
	let ci = a - p;
	println!("The compound interest is {}", ci);
}