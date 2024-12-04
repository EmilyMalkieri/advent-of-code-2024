use regex;

/// Parse numbers from a string
pub fn into_unsigned_nums(input: &str) -> impl Iterator<Item = u32> + use<'_> {
	input.split_whitespace().map(|s| {
		s.parse::<u32>()
			.unwrap_or_else(|_| panic!("Not a number: {s}"))
	})
}

/// Parse our input into multiple instances of a type T as found by a regular expression.
pub fn through_regex<'s, 'r, T, F>(
	input: &'s str,
	regex: &'r regex::Regex,
	parser: F,
) -> impl Iterator<Item = T> + use<'s, 'r, T, F>
where
	F: Fn(regex::Captures) -> Option<T>,
{
	regex.captures_iter(input).filter_map(parser)
}
