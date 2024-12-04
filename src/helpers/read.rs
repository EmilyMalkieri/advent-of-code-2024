use std::{
	fmt::Display,
	fs::File,
	io::{self, BufRead, BufReader},
	path::Path,
};

use super::parse_each_line;

/// Read in our input file as individual lines
pub fn to_lines<P>(path: P) -> io::Lines<BufReader<File>>
where
	P: Display,
	P: AsRef<Path>,
{
	let file = File::open(&path).unwrap_or_else(|_| panic!("Couldn't open the file {path}"));
	BufReader::new(file).lines()
}

/// Read in our input file as just one blob
pub fn to_string<P>(path: P) -> String
where
	P: Display,
	P: AsRef<Path>,
{
	std::fs::read_to_string(&path)
		.unwrap_or_else(|_| panic!("Couldn't open or read the file {path}"))
}

/// Read our input file as individual lines and parse each line into a type, via intermediate conversion into unsigned numbers.
pub fn and_parse_each_line_into_type<P, T>(path: P) -> impl Iterator<Item = T>
where
	P: Display,
	P: AsRef<Path>,
	T: for<'a> From<&'a [u32]>,
{
	parse_each_line::into_type(to_lines(path))
}
