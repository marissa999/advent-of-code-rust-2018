mod day01;
use day01::day01;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	// check if program has 1 arguments
	if args.len() < 2 {
		// not enough arguments
		println!("You need to specify at least 1 arguments");
		return;
	}

	if args.len() > 4 {
		// too many arguments, output warning
		println!("You specified too many arguments. Some arguments will get ignored!");
	}

	// parse day
	let parse_day = (&args[1]).parse::<i32>();	
	if !parse_day.is_ok(){
		// there was en error while parsing the day
		println!("Could not parse the day!");
		return;		
	}

	let day: i32 = (&args[1]).parse::<i32>().unwrap();

	let input = &args[2];

	// check if day is valid
	if !(day >= 1 && day <= 24) {
		// day invalid
		println!("Invalid day entered!");
		return;
	}

	// check if input file was specified
	if input == "" {
		// no input specified
		println!("No input file specified!");
		return;
	}

		println!("Day: {}", day);
	println!("Input: {}", input);

	if day == 1 {
		day01(input);
	}
}