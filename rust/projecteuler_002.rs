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

	//To find the "next_num(ber)" in a fibonacci sequence,
	//you take the current number (n) and add the previous number,
	// (n-1). These variables are as follows

	let mut n = 1;
	let mut n_minus_one = 0;
	let mut next_num;
	let mut total = 0;

	loop {
		//Fibonacci next number = n + (n-1)
		//Loop until next number is below 4 million
		next_num = n + n_minus_one;
		if next_num > 4000000 { break; }

		//Even numbers, using modulus %
		if next_num % 2 == 0 {
			total += next_num;
		}

		//Move forward a step, n becomes (n-1), 
		//and (n+1) becomes n
		n_minus_one = n;
		n = next_num;
	}

	//Print the total at the end
	println!("Total: {}", total);
}