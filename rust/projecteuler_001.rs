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

	//Define the starting number 32-bit integer
	//Starting number is 3 since 1 or 2 do not matter (multiples start at 3)
	let mut c = 3i32;

	//define the total sum as 64-bit integer to fit a larger number
	let mut total = 0i64;

	//While c is less than 1000 (incrememting by 1 each iteration)
	//check if c is divisible by 3 or 5, and add to the total
	while c < 1000 {
		if c % 3 == 0 || c % 5 == 0 {
			total += c as i64;
		}
		c += 1;
	}

	//Print statement for the answer. {} is a placeholder in rust
	println!("Sum of multiples of 3 and 5 less than 1000:\n{}\n", total);
}