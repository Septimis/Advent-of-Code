use std::io::{BufRead, BufReader};
use std::fs::File;

fn main()
{
	let (left_list, right_list) = read_input()
		.expect("Unable to process data into lists...");

	let mut distance_sum : usize = 0;
	let mut similarity_score : usize = 0;
	for (left_value, right_value) in left_list.iter().zip(right_list.iter())
	{
		distance_sum += left_value.abs_diff(*right_value);
	}

	for value in left_list
	{
		similarity_score += value * instances_found(value, &right_list);
	}

	println!("Total distance: {distance_sum}");
	println!("Similarity Score: {similarity_score}");
}

fn instances_found(value : usize, list : &Vec<usize>) -> usize
{
	let instance_index : Option<usize> = binary_search(list, 0, list.len() - 1, value);

	return match instance_index
	{
		None => 0,
		Some(index) =>
		{
			let mut instances : usize = 1;

			let mut forward_index : usize = index + 1;
			let mut backward_index : usize = index - 1;

			while list[forward_index] == value && forward_index < list.len()
			{
				instances += 1;
				forward_index += 1;
			}

			while list[backward_index] == value
			{
				instances += 1;

				if backward_index == 0 { break; }
				
				backward_index -= 1;
			}

			instances
		}
	};
}

// returns the index of the found value in the array
fn binary_search(list : &Vec<usize>, low : usize, high : usize, value : usize) -> Option<usize>
{
	if low > high
	{
		return None;
	}

	let mid : usize = low + (high - low) / 2;

	if value == list[mid]
	{
		return Some(mid);
	}

	if value < list[mid]
	{
		return binary_search(list, low, mid - 1, value);
	}

	return binary_search(list, mid + 1, high, value);
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