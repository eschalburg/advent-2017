use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
	let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

	let mut numbers: Vec<i32> = contents.split_whitespace().map(|n| n.parse().unwrap()).collect();

	let mut pos: i32 = 0;
	let mut steps: i32 = 0;
	while (pos as usize) < numbers.len() {
		//println!("Current vector {:?}", numbers);

		let instr = &mut numbers[pos as usize];

		pos = pos + *instr;

		*instr += 1;

		steps += 1;
	}

	println!("Total steps: {} ", steps);
}

fn part_2() {
	let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

	let mut numbers: Vec<i32> = contents.split_whitespace().map(|n| n.parse().unwrap()).collect();

	let mut pos: i32 = 0;
	let mut steps: i32 = 0;
	let mut old_jump: i32;

	while (pos as usize) < numbers.len() {
		let instr = &mut numbers[pos as usize];

		old_jump = *instr;

		if old_jump >= 3 {
			*instr -= 1;
		} else {
			*instr += 1;
		}
		pos = pos + *instr;

		steps += 1;
	}

	println!("Total steps: {} ", steps);
}