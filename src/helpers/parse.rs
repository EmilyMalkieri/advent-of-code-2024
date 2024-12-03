use regex;

/// Parse numbers from a string
pub fn into_unsigned_nums(input: &str) -> Vec<u32> {
	input
		.split_whitespace()
		.map(|s| u32::from_str_radix(s, 10).expect(&format!("Not a number: {}", s)))
		.collect()
}

/// Parse our input into multiple instances of a type T as found by a regular expression.
pub fn through_regex<T, F>(input: &str, regex: &regex::Regex, parser: F) -> Vec<T>
where
	F: Fn(regex::Captures) -> Option<T>,
{
	regex.captures_iter(input).filter_map(parser).collect()
}
