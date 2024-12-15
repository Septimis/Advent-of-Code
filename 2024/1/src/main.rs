use std::io::{BufRead, BufReader};
use std::fs::File;

fn main()
{
	let (left_list, right_list) = read_input()
		.expect("Unable to process data into lists...");

	let mut distance_sum : usize = 0;
	for (left_value, right_value) in left_list.iter().zip(right_list.iter())
	{
		distance_sum += left_value.abs_diff(*right_value);
	}

	println!("Total distance: {distance_sum}");
}

fn read_input() -> Option<(Vec<usize>, Vec<usize>)>
{
	let input_file : File = File::open("input.txt")
		.expect("Unable to open input.txt");
	let reader : BufReader<File> = BufReader::new(input_file);

	let mut left_list : Vec<usize> = Vec::new();
	let mut right_list : Vec<usize> = Vec::new();

	for line in reader.lines()
	{
		let line : String = line.expect("Expected to extract line");
		if line.trim().is_empty() { continue; }

		let line_parts : [usize; 2] = line
			.split_whitespace()
			.map(|number| number.parse::<usize>()
				.expect("Unable to parse string into usize...")
			)
			.collect::<Vec<usize>>()
			.try_into()
			.expect("Unable to convert to vector");

		insert(line_parts[0], &mut left_list);
		insert(line_parts[1], &mut right_list);
	}

	return Some((left_list, right_list));
}

fn insert(value : usize, list : &mut Vec<usize>)
{
	let mut insertion_index : usize = 0;

	for item in list.iter()
	{
		if *item > value { break; }

		insertion_index += 1;
	}

	list.insert(insertion_index, value);
}