use std::fs::File;
use std::io::{BufReader, BufRead};

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

		num_safe_reports += if check_safety(levels) { 1 } else { 0 }
	}

	println!("Safe Reports: {num_safe_reports}");
}

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
fn check_safety(levels : Vec<usize>) -> bool
{
	if levels.len() < 2 { return true; }

	let is_ascending : bool = match levels.as_slice()
	{
		[first, second, ..] if first < second => true,
		[first, second, ..] if first > second => false,
		_ => return false
	};

	let mut last : usize = levels[0];
	for level in levels.iter().skip(1)
	{
		if is_ascending && (*level <= last || level.abs_diff(last) > 3)
		{
			return false;
		}

		if !is_ascending && (*level >= last || level.abs_diff(last) > 3)
		{
			return false;
		}

		last = *level;
	}

	return true;
}