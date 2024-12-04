use core::num::ParseIntError;
use std::{
	fs::File,
	io::{self, BufReader},
};

pub mod parse;
pub mod parse_each_line;
pub mod read;
pub mod types;

/// Split our input line-by-line into two lists, separated by whitespace.
pub fn split_left_right(input: io::Lines<BufReader<File>>) -> (Vec<u32>, Vec<u32>) {
	let mut left = vec![];
	let mut right = vec![];
	for line in input {
		if let Some((l, r)) =
			left_right(&line.expect("Couldn't read line")).expect("Couldn't parse these numbers")
		{
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
		let left = first.parse::<u32>()?;
		let right = second.parse::<u32>()?;
		Ok(Some((left, right)))
	} else {
		Ok(None)
	}
}
