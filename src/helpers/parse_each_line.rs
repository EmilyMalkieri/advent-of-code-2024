use std::io::{BufRead, Lines};

use super::parse;

/// Parse each line into numbers.
fn into_unsigned_nums<L: BufRead>(lines: Lines<L>) -> impl Iterator<Item = Vec<u32>> {
	lines
		.map(|line| parse::into_unsigned_nums(&line.expect("Unable to retrieve this line")).collect())
}

pub fn into_type<T: for<'a> From<&'a [u32]>, L: BufRead>(
	lines: Lines<L>,
) -> impl Iterator<Item = T> {
	into_unsigned_nums(lines).map(|line| T::from(&line))
}
