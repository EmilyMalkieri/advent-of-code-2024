use core::iter;
use std::collections::HashMap;

use crate::helpers;

fn distance(a: u32, b: u32) -> u32 {
	a.abs_diff(b)
}

fn total_distance(left: &[u32], right: &[u32]) -> u32 {
	iter::zip(left, right).map(|(&a, &b)| distance(a, b)).sum()
}

/// Count how often a given element appears in a list.
fn count_occurrences(list: &[u32], elem: u32) -> u32 {
	list
		.iter()
		.filter(|&&e| e == elem)
		.count()
		.try_into()
		.expect("Couldn't convert from usize")
}

/// Calculate the "similarity score" between both lists
fn calculate_similarity_score(left: &[u32], right: &[u32]) -> u32 {
	let mut cache = HashMap::new();
	left
		.iter()
		.map(|num| {
			num * *cache
				.entry(num)
				.or_insert_with(|| count_occurrences(right, *num))
		})
		.sum()
}

#[allow(dead_code)]
pub fn solve_1() -> u32 {
	let input = helpers::read::to_lines("inputs/day01/input.txt");
	let (mut left, mut right) = helpers::split_left_right(input);
	left.sort_unstable();
	right.sort_unstable();
	total_distance(&left, &right)
}

#[allow(dead_code)]
pub fn solve_2() -> u32 {
	let input = helpers::read::to_lines("inputs/day01/input.txt");
	let (left, right) = helpers::split_left_right(input);
	calculate_similarity_score(&left, &right)
}

#[cfg(test)]
mod tests {
	use crate::{
		day01_historian_hysteria::{calculate_similarity_score, total_distance},
		helpers,
	};

	#[test]
	fn ex01() {
		let input = helpers::read::to_lines("inputs/day01/ex01.txt");
		let (mut left, mut right) = helpers::split_left_right(input);
		left.sort_unstable();
		right.sort_unstable();
		assert_eq!(Some(&1), left.first());
		assert_eq!(Some(&3), right.first());
		assert_eq!(11, total_distance(&left, &right));
	}

	#[test]
	fn ex02() {
		let input = helpers::read::to_lines("inputs/day01/ex01.txt");
		let (left, right) = helpers::split_left_right(input);
		assert_eq!(31, calculate_similarity_score(&left, &right));
	}
}
