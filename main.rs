use std::io::*;
fn main() {
	let mut input = BufReader::new(std::io::stdin());
	let mut buf = String::new();
	let line = input.read_line(&mut buf);
	let xs: Vec<&str> = buf.trim().split(' ').collect();
	dbg!(&xs);
	for i in 0..2 {
		let n: f64 = xs[i].parse().unwrap();
		dbg!(n);
	}
}