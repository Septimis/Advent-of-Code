use std::fs::File;
use std::io::{BufReader, BufRead};

// 544 & 548 & 559 too low

fn main()
{
	let input_file : File = File::open("input.txt")
		.expect("Unable to open input file...");
	let reader : BufReader<File> = BufReader::new(input_file);
	
	let mut num_safe_reports : usize = 0;
	for report in reader.lines()
	{
		let report : String = report.expect("Unable to read line in input...");
		
		if report.trim().is_empty() { continue; }

		let levels : Vec<usize> = report
			.split_whitespace()
			.map(|level| level.parse::<usize>().expect("Unable to parse number into usize..."))
			.collect::<Vec<usize>>()
			.try_into()
			.expect("Unable to parse report into vector of usize's...");

		num_safe_reports += 
		if check_safety(levels, 1) { 1 } else { 0 };
	}

	println!("Safe Reports: {num_safe_reports}");
}

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
fn check_safety(mut levels : Vec<usize>, problem_dampener_charges : usize) -> bool
{
	if levels.len() < 2 { return true; }

	let is_ascending : bool = match levels.as_slice()
	{
		[first, second, ..] if first < second => true,
		[first, second, ..] if first > second => false,
		_ =>
		{
			if problem_dampener_charges > 0
			{
				levels.remove(0);
				return check_safety(levels, problem_dampener_charges - 1);
			}

			return false;
		}
	};

	let mut last : usize = levels[0];
	for &level in levels.iter().skip(1)
	{
		let is_unsafe : bool =
			(is_ascending && level <= last) ||
			(!is_ascending && level >= last) ||
			level.abs_diff(last) > 3;

		if is_unsafe
		{
			if problem_dampener_charges > 0
			{
				for index in 0 .. levels.len()
				{
					let mut levels_copy : Vec<usize> = levels.clone();
					levels_copy.remove(index);

					if check_safety(levels_copy, problem_dampener_charges - 1) { return true; }
				}
			}

			return false;
		}

		last = level;
	}

	return true;
}