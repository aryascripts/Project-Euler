/*
Project Euler Problem #003 - Largest prime factor
https://projecteuler.net/problem=3

Prompt: 
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143?

Code by Aman Bhimani
Written in Rust
https://www.rust-lang.org/

24 May 2016
*/

fn main() {
	//number is the target number from Prompt
	//primes is a vector including the first prime number
	let mut number = 600851475143i64;
	let mut primes = vec![2];
	let mut last_prime = 0;

	/* checks if the "number" is divisible by the last prime 
	number in the vector "primes". If it is divisible, it divides
	and gets a new "number". Repeat the process until the "new"
	number is a prime number itself. This is the largest prime factor. */
	loop {
		last_prime = primes[primes.len()-1];

		if number % last_prime == 0 {
			number = number / last_prime;
			if is_prime(number) {
				break;
			}
		}
		//Stores the prime numbers found til now in a vector
		primes.push(next_prime(&last_prime));
	}

	println!("Largest prime factor: {}", number);
}

//takes in an integer and finds
//the next prime number in sequence
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

//prime numbers are only diviible by 1 and the number itself.
// the if statement checks if argument is divisible by any other
// number other than the number itself. If so, returns false (not prime)
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