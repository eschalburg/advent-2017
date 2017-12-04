use std::io;
use std::f32;

fn main() {
	part_1();
	part_2();
}

fn part_1() {
    println!("Enter input string");

    let mut input = String::new();

	io::stdin().read_line(&mut input).expect("Failed to get input.");

	let input_opt = input.trim().parse::<i32>();

	let input_num = match input_opt {
		Ok(input_num) => input_num,
		Err(e) => {
			println!("{} is not a number, ({})", input.trim(), e);
			return;
		}
	};

	println!("Finding {}", input_num);

	//Find closest square number
	let root_round: i32 = (input_num as f32).sqrt().round() as i32;

	let floor = root_round;
	let ceiling = root_round + 1;

	let floor_sq = floor * floor;
	let ceiling_sq = ceiling * ceiling;

	let floor_dist = input_num - floor_sq;
	let ceiling_dist = ceiling_sq - input_num;

	let closest_sq;
	if floor_dist < ceiling_dist {
		closest_sq = floor_sq;
	} else {
		closest_sq = ceiling_sq;
	}

	//How far am I from the closest square?
	let mut sq_dist = ((input_num - closest_sq) as f32).abs() as i32;

	//If I am a sqaure, my own root - 1 is the distance.  Otherwise, it's my distance from a square.
	if closest_sq == input_num {
		sq_dist = root_round - 1;
	    println!("Closest square is {} for {}, distance to center is {}", closest_sq, input_num, sq_dist);
	} else {
	    println!("Closest square is {} for {}, distance to center is {}", closest_sq, input_num, sq_dist);
	}
}

fn part_2() {
	
}
