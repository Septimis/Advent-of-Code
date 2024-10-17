use std::fs;
use std::io::{self, BufRead};

const FILE_PATH : &str = "input.txt";

fn main() {
	let lines : Vec<String> = read_file().expect("Unable to read file...");

	let mut total : usize = 0;

	for line in lines
	{
		let literal : usize = line.chars().count();
		let memory : usize = extract_num_chars_in_memory(&line);

		total += literal - memory;
	}

	println!("Total: {total}");
}

fn extract_num_chars_in_memory(line : &str) -> usize
{
	let mut memory_chars : usize = 0;
	let mut skip_num : u8 = 0;

	for (i, c) in line.chars().enumerate()
	{
		if skip_num > 0
		{
			skip_num -= 1;
			continue;
		}

		if c == '\\'
		{
			if line.as_bytes()[i + 1] == b'x'
			{
				skip_num += 3;
			}
			else
			{
				skip_num += 1;
			}
		}

		memory_chars += 1;
	}

	return memory_chars - 2;
}

fn read_file() -> Result<Vec<String>, io::Error>
{
	let file : fs::File = fs::File::open(FILE_PATH)?;
	let file_reader : io::BufReader<fs::File> = io::BufReader::new(file);

	let lines : Vec<String> = file_reader.lines()
		.filter_map(Result::ok)
		.filter(|line| !line.is_empty())
		.collect();

	return Ok(lines);
}