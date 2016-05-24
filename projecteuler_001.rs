/*
Project Euler Problem #001 - Multiples of 3 and 5
https://projecteuler.net/problem=1

Prompt: 
If we list all the natural numbers below 10 that 
are multiples of 3 or 5, we get 3, 5, 6 and 9. 
The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.

Code by Aman Bhimani
Written in Rust
https://www.rust-lang.org/

22 May 2016
*/

fn main() {

	let mut c = 3i64;
	let mut total = 0i64;

	while c < 1000 {
		if c % 3 == 0 || c % 5 == 0 {
			total += c;
		}
		c += 1;
	}

	println!("Sum of multiples of 3 and 5 less than 1000:\n{}\n", total);
}