use std::io::{BufRead, Lines};

use super::parse;

/// Parse each line into numbers.
fn into_unsigned_nums<L: BufRead>(lines: Lines<L>) -> impl Iterator<Item = Vec<u32>> {
	lines.map(|line| {
		parse::into_unsigned_nums_by_whitespace(&line.expect("Unable to retrieve this line"))
			.collect()
	})
}

pub fn into_type<T: for<'a> From<&'a [u32]>, L: BufRead>(
	lines: Lines<L>,
) -> impl Iterator<Item = T> {
	into_unsigned_nums(lines).map(|line| T::from(&line))
}

pub fn by<B, F, T>(lines: Lines<B>, func: F) -> impl Iterator<Item = T>
where
	B: BufRead,
	F: Fn(String) -> T,
{
	lines.map(move |line| func(line.expect("Should have been able to read this line.")))
}
