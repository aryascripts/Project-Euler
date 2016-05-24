/*
Project Euler Problem #002 - Even Fibonacci numbers
https://projecteuler.net/problem=2

Prompt: 
Each new term in the Fibonacci sequence is generated 
by adding the previous two terms. By starting with 
1 and 2, the first 10 terms will be:

1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

By considering the terms in the Fibonacci sequence 
whose values do not exceed four million, find the sum 
of the even-valued terms.

Code by Aman Bhimani
Written in Rust
https://www.rust-lang.org/

23 May 2016
*/

fn main() {
	let mut n = 1;
	let mut n_minus_one = 0;
	let mut next_num;
	let mut total = 0;

	loop {
		next_num = n + n_minus_one;
		if next_num > 4000000 { break; }

		if next_num % 2 == 0 {
			total += next_num;
		}

		n_minus_one = n;
		n = next_num;
	}

	println!("Total: {}", total);
}