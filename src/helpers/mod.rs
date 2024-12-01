use std::{
	fs::File,
	io::{self, BufRead, BufReader},
	num::ParseIntError,
	path::Path,
};

/// Read in our input file
pub fn read_input<T: AsRef<Path>>(path: T) -> io::Result<io::Lines<BufReader<File>>> {
	let file = File::open(&path)?;
	Ok(BufReader::new(file).lines())
}

/// Split our input line by line into two lists, separated by whitespace.
pub fn split_left_right(input: io::Lines<BufReader<File>>) -> (Vec<u32>, Vec<u32>) {
	let mut left = vec![];
	let mut right = vec![];
	for line in input {
		if let Some((l, r)) = left_right(&line.unwrap()).unwrap() {
			left.push(l);
			right.push(r);
		};
	}
	(left, right)
}

/// Parse a line of two values, separated by whitespace.
fn left_right(line: &str) -> Result<Option<(u32, u32)>, ParseIntError> {
	let mut it = line.split_whitespace();
	if let Some(first) = it.next()
		&& let Some(second) = it.next()
	{
		let left = u32::from_str_radix(first, 10)?;
		let right = u32::from_str_radix(second, 10)?;
		return Ok(Some((left, right)));
	} else {
		return Ok(None);
	}
}
