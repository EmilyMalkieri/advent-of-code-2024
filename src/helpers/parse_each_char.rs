use unicode_segmentation::UnicodeSegmentation as _;

pub fn zero_indexed<F, T>(input: &str, func: F) -> Vec<T>
where
	F: Fn(&str, usize, usize) -> T,
{
	input
		.lines()
		.enumerate()
		.flat_map(|(row_idx, line)| {
			line
				.graphemes(true)
				.enumerate()
				.map(move |(col_idx, s)| (s, row_idx, col_idx))
		})
		.map(|(s, row_idx, col_idx)| func(s, row_idx, col_idx))
		.collect()
}

pub fn non_indexed<F, T>(input: &str, func: F) -> Vec<T>
where
	F: Fn(&str) -> T,
{
	zero_indexed(input, |s, _, _| func(s))
}

pub fn into_digits(input: &str) -> Vec<u8> {
	non_indexed(input.trim(), |s| {
		s.parse::<u8>().expect("Should have been a digit.")
	})
}
