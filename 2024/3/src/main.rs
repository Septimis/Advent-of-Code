use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main()
{
	let input_file : File = File::open("input.txt").expect("Expected to read input.txt...");
	let reader : BufReader<File> = BufReader::new(input_file);

	let mul_regex : Regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").expect("Expected to create regex...");

	let mut result : usize = 0;
	let mut is_enabled : bool = true;
	for input_line in reader.lines()
	{
		let input_line : String = input_line.expect("Expected to read line to String...");

		if input_line.trim().is_empty() { continue; }

		for instance in mul_regex.find_iter(&input_line)
		{
			match instance.as_str()
			{
				"don't()" =>
				{
					is_enabled = false;
				},

				"do()" =>
				{
					is_enabled = true;
				},

				_ =>
				{
					if !is_enabled { continue; }

					let left_operand : usize = instance.as_str()
						[4 .. instance.as_str().find(',').expect("Expected to find a comma in mul() statement...")]
						.parse::<usize>()
						.expect("Expected to parse into usize...");
	
					let right_operand : usize = instance.as_str()
						[instance.as_str().find(',').expect("Expected to find comma in mul() statement...") + 1 .. instance.len() - 1]
						.parse::<usize>()
						.expect("Expected to parse into usize...");
		
					result += left_operand * right_operand;
				}
			};
		}
	}

	println!("Final sum: {result}");
}
