use std::{collections::HashMap, usize};

const FILE_PATH : &str = "data/input.txt";

// ATTEMPT 1 PART 2 (too low): 33706

fn main()
{
	// Maps an identifier to its signal
	let mut wires : HashMap<String, u16> = HashMap::new();

	// All lines in the file which have not been acted on
	let mut lines : Vec<String> = Vec::new();
	
	read_input(&mut lines);

	while !lines.is_empty()
	{
		lines.swap_remove(attach_wire(&mut wires, &lines, false)
			.expect("Expected a valid line. Got None...")
		);
	}

	let temp : u16 = wires["a"];
	wires.clear();
	wires.insert(String::from("b"), temp);

	read_input(&mut lines);

	while !lines.is_empty()
	{
		lines.swap_remove(attach_wire(&mut wires, &lines, true)
			.expect("Expected a valid line. Got None...")
		);
	}

	println!("Wire 'a': '{}'", wires["a"]);
}

// Provides the index of the line successfully processed into the wires HashMap
fn attach_wire(wires : &mut HashMap<String, u16>, lines : &Vec<String>, ignore_input_to_b : bool) -> Option<usize>
{
	for (index, line) in lines.iter().enumerate()
	{
		let line_parts : Vec<&str> = line.split_ascii_whitespace().collect();

		if ignore_input_to_b && *line_parts.last().unwrap() == "b"
		{
			return Some(index);
		}

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