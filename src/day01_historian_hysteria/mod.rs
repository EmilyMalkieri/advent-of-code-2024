use std::iter;

use crate::helpers;

fn distance(a: u32, b: u32) -> u32 {
	a.abs_diff(b)
}

fn total_distance(left: &[u32], right: &[u32]) -> u32 {
	iter::zip(left, right).map(|(&a, &b)| distance(a, b)).sum()
}

pub fn solve_1() -> u32 {
	let input = helpers::read_input("inputs/day01/input.txt").unwrap();
	let (mut left, mut right) = helpers::split_left_right(input);
	left.sort();
	right.sort();
	total_distance(&left, &right)
}

#[cfg(test)]
mod tests {
	use crate::{day01_historian_hysteria::total_distance, helpers};

	#[test]
	pub fn ex01() {
		let input = helpers::read_input("inputs/day01/ex01.txt").unwrap();
		let (mut left, mut right) = helpers::split_left_right(input);
		left.sort();
		right.sort();
		assert_eq!(Some(&1), left.first());
		assert_eq!(Some(&3), right.first());
		assert_eq!(11, total_distance(&left, &right));
	}
}
