// here i'm using the so called bruteForce method cause it's the one i'm familiar with
// takes O(n**2) time complexity, so for larger numbers it will cost a lot of time
pub fn nth(n: u32) -> u32 {
	if n == 0 {return 2} // special case, cause the caunting on the test.rs start from 0
	
	let mut iterator = 0;
	let mut prime = 2;
	
	while iterator != n {
		prime += 1;
		if is_prime(prime) == true {
			iterator += 1;	
		}
	}
	prime
}

pub fn is_prime(number: u32) -> bool {
	if number == 2 {return true}
	for x in 2..number {
		if number % x == 0 {
			return false
		}
	}
	return true
}
