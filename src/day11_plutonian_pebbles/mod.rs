use num::Integer as _;

use crate::helpers;

type Num = u64;

#[allow(dead_code)]
pub fn solve_1() -> usize {
	let mut stones: Vec<Num> = helpers::parse::into_unsigned_nums_by_whitespace(
		&helpers::read::to_string("inputs/day11/input.txt"),
	)
	.collect();

	for _ in 1..=25 {
		stones = stones.into_iter().flat_map(blink_naïve).collect();
	}
	stones.len()
}

#[allow(dead_code)]
pub fn solve_2() -> usize {
	todo!()
}

fn blink_naïve(stone: Num) -> Vec<Num> {
	if stone == 0 {
		vec![1]
	} else {
		let read = stone.to_string();
		if read.len().is_even() {
			let (a, b) = read.split_at(read.len() / 2);
			vec![
				a.parse()
					.expect("We just split this in twine, it should fit."),
				b.parse()
					.expect("We just split this in twine, it should fit."),
			]
		} else {
			vec![2024 * stone]
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Num;
	use crate::{day11_plutonian_pebbles::blink_naïve, helpers};

	#[test]
	fn ex01() {
		let stones: Vec<Num> = helpers::parse::into_unsigned_nums_by_whitespace(
			&helpers::read::to_string("inputs/day11/ex01.txt"),
		)
		.collect();

		let actual: Vec<_> = stones.into_iter().flat_map(blink_naïve).collect();
		let expected = vec![1, 2024, 1, 0, 9, 9, 2_021_976];
		assert_eq!(expected, actual);
	}

	#[test]
	fn ex02() {
		let mut stones: Vec<Num> = helpers::parse::into_unsigned_nums_by_whitespace(
			&helpers::read::to_string("inputs/day11/ex02.txt"),
		)
		.collect();

		let expected_after_each_blink = vec![
			vec![253_000, 1, 7],
			vec![253, 0, 2024, 14168],
			vec![512_072, 1, 20, 24, 28_676_032],
			vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
			vec![1_036_288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
			vec![
				2_097_446_912,
				14168,
				4048,
				2,
				0,
				2,
				4,
				40,
				48,
				2024,
				40,
				48,
				80,
				96,
				2,
				8,
				6,
				7,
				6,
				0,
				3,
				2,
			],
		];
		for expected in expected_after_each_blink {
			stones = stones.into_iter().flat_map(blink_naïve).collect();
			assert_eq!(expected, stones);
		}
		let expected_after_6 = 22;
		assert_eq!(expected_after_6, stones.len());
		let expected_after_25 = 55312;
		for _ in 1..=(25 - 6) {
			stones = stones.into_iter().flat_map(blink_naïve).collect();
		}
		assert_eq!(expected_after_25, stones.len());
	}
}
