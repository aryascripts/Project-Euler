/*
Project Euler Problem #002 - Even Fibonacci numbers
https://projecteuler.net/problem=2

Prompt: 
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143?

Code by Aman Bhimani
Written in Rust
https://www.rust-lang.org/

24 May 2016
*/

fn main() {
	let mut number = 600851475143i64;
	let mut primes = vec![2];
	// let prime_factors = vec![1];
	let mut last_prime = 0;

	loop {
		last_prime = primes[primes.len()-1];
		
		if number % last_prime == 0 {
			number = number / last_prime;

			if is_prime(number) {
				break;
			}
		}
		primes.push(next_prime(&last_prime));
	}

	println!("Largest prime factor: {}", number);
}


fn next_prime(x: &i64) -> i64 {
	let mut s = *x;
	let mut ret = 0;
	loop {
		s += 1;
		if is_prime(s) {
			ret = s;
			break;
		}
	}
	ret
}

fn is_prime(x: i64) -> bool {
	let mut prime = true;
	for y in 2..x {
		if x % y == 0 && y != x {
			prime = false;
			break;
		}
	}
	prime
}