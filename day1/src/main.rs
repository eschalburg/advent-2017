use std::io;

fn main() {
	let mut input = String::new();
	const RADIX: u32 = 10;

	println!("Enter input string");

	io::stdin().read_line(&mut input).expect("Failed to get input.");

    let len = input.trim().len();
    let jump = len/2;

    println!("Input: {}", input);
    println!("{} characters.", len);

    let mut sum: u32 = 0;

	for ind in 0..(len) {	
		if jump + ind > (len - 1) {
			// Figure out offset for wraparound
			let offset = jump - (len - ind);
			// println!("Ind: {}, Len: {}, Jump: {}, Offset: {}", ind, len, jump, offset);

			let s1 = input.chars().nth(ind).unwrap().to_digit(RADIX).unwrap();
		  	let s2 = input.chars().nth(offset).unwrap().to_digit(RADIX).unwrap(); 
			// println!("Comparing {} to {}", s1, s2);

			if s1 == s2 {
				sum = sum + s1;
			}
		} else {
			let s1 = input.chars().nth(ind).unwrap().to_digit(RADIX).unwrap();
			let s2 = input.chars().nth(ind + jump).unwrap().to_digit(RADIX).unwrap();
			// println!("Comparing {} to {}", s1, s2);

			if s1 == s2 {
				sum = sum + s1;
			}
		}
	}

	println!("Sum is {}", sum);
}
