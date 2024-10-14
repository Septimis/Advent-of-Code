use std::collections::HashMap;

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

		match line_parts.as_slice()
		{
			["NOT", input_identifier, _, output_identifier] =>
			{
				if let Some(signal_one) = get_signal(&input_identifier, &wires)
				{
					wires.insert(output_identifier.to_string(), !signal_one);
					return Some(index);
				}
			}

			// Pattern for [input operation input arrow output]
			[input_identifier_left, operation, input_identifier_right, _, output_identifier] =>
			{
				if let (Some(input_left), Some(input_right)) = (get_signal(&input_identifier_left, &wires), get_signal(&input_identifier_right, &wires))
				{
					let result = match *operation
					{
						"AND"	=> input_left & input_right,
						"OR"	=> input_left | input_right,
						"LSHIFT"=> input_left << input_right,
						"RSHIFT"=> input_left >> input_right,
						_		=> unreachable!(),
					};

					wires.insert(output_identifier.to_string(), result);
					return Some(index);
				}
			}

			// Direct input
			[input_identifier, _, output_identifier] =>
			{
				if let Some(input_signal) = get_signal(&input_identifier, &wires)
				{
					wires.insert(output_identifier.to_string(), input_signal);
					return Some(index);
				}
			}

			_ => panic!("Unable to recognize line {}", line),
		}
	}

	return None;
}

// Returns the signal value in a u16 Option
fn get_signal(signal : &str, wires : &HashMap<String, u16>) -> Option<u16>
{
	return wires.get(signal).copied().or_else(|| signal.parse().ok());
}

fn read_input(lines : &mut Vec<String>)
{
	let file_content : String = std::fs::read_to_string(FILE_PATH)
		.expect("Unable to read file...");

	lines.extend(file_content.lines().filter(|line| !line.is_empty()).map(String::from));
}