use std::{collections::HashMap, usize};

const FILE_PATH : &str = "data/input.txt";

fn main()
{
	// Maps an identifier to its signal
	let mut wires : HashMap<String, u16> = HashMap::new();

	// All lines in the file which have not been acted on
	let mut lines : Vec<String> = Vec::new();
	
	read_input(&mut lines);

	while !lines.is_empty()
	{
		lines.swap_remove(attach_wire(&mut wires, &lines)
			.expect("Expected a valid line. Got None...")
		);
	}

	println!("The value at 'a' is '{}'", wires["a"]);
}

// Provides the index of the line successfully processed into the wires HashMap
fn attach_wire(wires : &mut HashMap<String, u16>, lines : &Vec<String>) -> Option<usize>
{
	for (index, line) in lines.iter().enumerate()
	{
		let line_parts : Vec<&str> = line.split_ascii_whitespace().collect();

		if line.contains("AND")
		{
			let signal_one : Option<u16> = get_signal(line_parts[0], &wires);
			let signal_two : Option<u16> = get_signal(line_parts[2], &wires);

			if signal_one.is_some() && signal_two.is_some()
			{
				wires.insert(String::from(*line_parts.last().unwrap()), signal_one.unwrap() & signal_two.unwrap());
				return Some(index);
			}
		}
		else if line.contains("OR")
		{
			let signal_one : Option<u16> = get_signal(line_parts[0], &wires);
			let signal_two : Option<u16> = get_signal(line_parts[2], &wires);

			if signal_one.is_some() && signal_two.is_some()
			{
				wires.insert(String::from(*line_parts.last().unwrap()), signal_one.unwrap() | signal_two.unwrap());
				return Some(index);
			}
		}
		else if line.contains("LSHIFT")
		{
			let signal_one : Option<u16> = get_signal(line_parts[0], &wires);
			let signal_two : Option<u16> = get_signal(line_parts[2], &wires);

			if signal_one.is_some() && signal_two.is_some()
			{
				wires.insert(String::from(*line_parts.last().unwrap()), signal_one.unwrap() << signal_two.unwrap());
				return Some(index);
			}
		}
		else if line.contains("RSHIFT")
		{
			let signal_one : Option<u16> = get_signal(line_parts[0], &wires);
			let signal_two : Option<u16> = get_signal(line_parts[2], &wires);

			if signal_one.is_some() && signal_two.is_some()
			{
				wires.insert(String::from(*line_parts.last().unwrap()), signal_one.unwrap() >> signal_two.unwrap());
				return Some(index);
			}
		}
		else if line.contains("NOT")
		{
			let signal_one : Option<u16> = get_signal(line_parts[1], &wires);

			if signal_one.is_some()
			{
				wires.insert(String::from(*line_parts.last().unwrap()), !signal_one.unwrap());
				return Some(index);
			}
		}
		else if line_parts.len() == 3
		{
			let signal_one : Option<u16> = get_signal(line_parts[0], &wires);

			if signal_one.is_some()
			{
				wires.insert(String::from(*line_parts.last().unwrap()), signal_one.unwrap());
				return Some(index);
			}
		}
		else
		{
			panic!("Did not recognize line {}", line);
		}
	}

	return None;
}

// Returns the signal value in a u16 Option
fn get_signal(signal : &str, wires : &HashMap<String, u16>) -> Option<u16>
{
	if wires.contains_key(signal) { return Some(wires[signal]); }
	
	return signal.parse::<u16>().ok();
}

fn read_input(lines : &mut Vec<String>)
{
	let file_content : String = std::fs::read_to_string(FILE_PATH)
		.expect("Unable to read file...");

	for line in file_content.lines()
	{
		if line.is_empty() { continue; }

		lines.push(String::from(line));
	}
}