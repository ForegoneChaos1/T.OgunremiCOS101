fn main() {
	let t:f64 = 225_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 250_000.00;
	let d:f64 = 950_000.00;
	let a:f64 = 250_000.00;

	let s = ( 2.0 * t ) + m + ( 3.0 *h ) + ( 3.0 * d ) + a;
	let av = s / 10.0;
	println!("The sum of the sales record is {:.2}", s);
	println!("The average of the sales record is {:.2}", av);
}