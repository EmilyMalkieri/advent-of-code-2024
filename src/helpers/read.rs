use std::{
	fmt::Display,
	fs::File,
	io::{self, BufRead, BufReader},
	path::Path,
};

use super::parse_each_line;

/// Read in our input file as individual lines
pub fn to_lines<P: AsRef<Path>>(path: P) -> io::Lines<BufReader<File>>
where
	P: Display,
{
	let file = File::open(&path).expect(&format!("Couldn't open the file {}", path));
	BufReader::new(file).lines()
}

/// Read our input file as individual lines and parse each line into a type, via intermediate conversion into unsigned numbers.
pub fn and_parse_each_line_into_type<P: AsRef<Path>, T: for<'a> From<&'a [u32]>>(
	path: P,
) -> impl Iterator<Item = T>
where
	P: Display,
{
	parse_each_line::into_type(to_lines(path))
}
