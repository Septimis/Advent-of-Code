use std::fs::File;
use std::io::{BufRead, BufReader};

// 7556 too high

fn main()
{
	let grid : Vec<Vec<char>> = read_input();

	let mut occurances : usize = 0;
	for (row_index, row) in grid.iter().enumerate()
	{
		for col_index in 0 .. row.len()
		{
			if grid[row_index][col_index] != 'X' { continue; }
			occurances += find_xmas(&grid, row_index, col_index);
		}
	}

	println!("{occurances} found");
}

// Takes the grid and position of 'X' and returns how many times "XMAS" is found
fn find_xmas(grid : &Vec<Vec<char>>, row_index : usize, col_index : usize) -> usize
{
	let mut occurances : usize = 0;

	let lower_row_index : usize = if row_index < 3 { 0 } else { row_index - 3 };
	let upper_row_index : usize = if row_index + 3 >= grid.len() { grid.len() - 1 } else { row_index + 4 };

	let lower_col_index : usize = if col_index < 3 { 0 } else { col_index - 3 };
	let upper_col_index : usize = if col_index + 3 >= grid[row_index].len() { grid[row_index].len() - 1} else { col_index + 4 };

	println!("Found 'X' on line {row_index} character {col_index}");
	for i in lower_row_index .. upper_row_index
	{
		for j in lower_col_index .. upper_col_index
		{
			print!("{} ", if i == row_index && j == col_index { '🗴' } else { grid[i][j] });
		}
		println!();
	}

	// look up
	if row_index >= 3 &&
		grid[row_index - 1][col_index] == 'M' &&
		grid[row_index - 2][col_index] == 'A' &&
		grid[row_index - 3][col_index] == 'S'
	{
		println!("\tUP");
		occurances += 1;
	}

	// look diagonally up & right
	if row_index >= 3 && col_index + 3 < grid[row_index].len() &&
		grid[row_index - 1][col_index + 1] == 'M' &&
		grid[row_index - 2][col_index + 2] == 'A' &&
		grid[row_index - 3][col_index + 3] == 'S'
	{
		println!("\tDIAGONALLY UP & RIGHT");
		occurances += 1;
	}

	// look right
	if col_index + 3 < grid[row_index].len() &&
		grid[row_index][col_index + 1] == 'M' &&
		grid[row_index][col_index + 2] == 'A' &&
		grid[row_index][col_index + 3] == 'S'
	{
		println!("\tRIGHT");
		occurances += 1;
	}

	// look diagonally down & right
	if row_index + 3 < grid.len() && col_index + 3 < grid[row_index].len() &&
		grid[row_index + 1][col_index + 1] == 'M' &&
		grid[row_index + 2][col_index + 2] == 'A' &&
		grid[row_index + 3][col_index + 3] == 'S'
	{
		println!("\tDIAGONALLY DOWN & RIGHT");
		occurances += 1;
	}

	// look down
	if row_index + 3 < grid.len() &&
		grid[row_index + 1][col_index] == 'M' &&
		grid[row_index + 2][col_index] == 'A' &&
		grid[row_index + 3][col_index] == 'S'
	{
		println!("\tDOWN");
		occurances += 1;
	}

	// look diagonally down & left
	if row_index + 3 < grid.len() && col_index >= 3 &&
		grid[row_index + 1][col_index - 1] == 'M' &&
		grid[row_index + 2][col_index - 2] == 'A' &&
		grid[row_index + 3][col_index - 3] == 'S'
	{
		println!("\tDIAGONALLY DOWN & LEFT");
		occurances += 1;
	}

	// look left
	if col_index >= 3 &&
		grid[row_index][col_index - 1] == 'M' &&
		grid[row_index][col_index - 2] == 'A' &&
		grid[row_index][col_index - 3] == 'S'
	{
		println!("\tLEFT");
		occurances += 1;
	}

	// look diagonally up and left
	if row_index >= 3 && col_index >= 3 &&
		grid[row_index - 1][col_index - 1] == 'M' &&
		grid[row_index - 2][col_index - 2] == 'A' &&
		grid[row_index - 3][col_index - 3] == 'S'
	{
		println!("\tDIAGONALLY UP & LEFT");
		occurances += 1;
	}
	println!("\n");
	return occurances
}

fn read_input() -> Vec<Vec<char>>
{
	let input_file : File = File::open("input.txt").expect("Expected to open input.txt");
	let reader : BufReader<File> = BufReader::new(input_file);

	let mut grid : Vec<Vec<char>> = Vec::new();
	for line in reader.lines()
	{
		let line : String = line.expect("Expected to extract string from line...");
		if line.trim().is_empty() { continue; }

		let mut row : Vec<char> = Vec::new();
		for c in line.chars()
		{
			row.push(c);
		}
		grid.push(row);
	}

	return grid;
}