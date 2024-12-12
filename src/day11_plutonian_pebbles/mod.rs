use std::collections::HashMap;

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
	let stones: Vec<Num> = helpers::parse::into_unsigned_nums_by_whitespace(
		&helpers::read::to_string("inputs/day11/input.txt"),
	)
	.collect();
	let mut memo = Memo::new();

	stones
		.into_iter()
		.map(|stone| blink_refined(stone, 75, &mut memo))
		.sum()
}

type Memo = HashMap<(Num, u8), usize>;

fn blink_refined(stone: Num, times: u8, memo: &mut Memo) -> usize {
	if times == 0 {
		unreachable!()
	}
	if let Some(count) = memo.get(&(stone, times)) {
		return *count;
	}
	if times == 1 {
		let easy = blink_naïve(stone).len();
		memo.insert((stone, times), easy);
		return easy;
	}
	let final_count: usize = blink_naïve(stone)
		.iter()
		.map(|st| blink_refined(*st, times - 1, memo))
		.sum();

	memo.insert((stone, times), final_count);
	final_count
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
	use super::Memo;
	use super::Num;
	use crate::day11_plutonian_pebbles::blink_refined;
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

	#[test]
	fn ex02_memoized() {
		let mut stones: Vec<Num> = helpers::parse::into_unsigned_nums_by_whitespace(
			&helpers::read::to_string("inputs/day11/ex02.txt"),
		)
		.collect();
		let mut memo = Memo::new();

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
		let expected_after_25: usize = 55312;
		let count = stones
			.into_iter()
			.map(|stone| blink_refined(stone, 25 - 6, &mut memo))
			.sum();
		assert_eq!(expected_after_25, count);
	}
}
