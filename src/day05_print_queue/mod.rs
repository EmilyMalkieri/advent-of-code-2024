use core::cmp;
use std::collections::{HashMap, HashSet};

use crate::helpers;

#[allow(dead_code)]
pub fn solve_1() -> u32 {
	let input = helpers::read::to_string("inputs/day05/input.txt");
	let (ordering, updates) = parse(&input);
	updates
		.iter()
		.filter(|update| update.is_sorted_by(|a, b| ordering.allowed_before(*a, *b)))
		.map(|update| middle_entry(update))
		.sum()
}

#[allow(dead_code)]
pub fn solve_2() -> u32 {
	todo!()
}

struct Ordering(HashMap<u32, HashSet<u32>>);

impl Ordering {
	fn depends_on(&self, a: u32, b: u32) -> bool {
		if let Some(direct_dependencies) = self.0.get(&a) {
			direct_dependencies.contains(&b)
			// Wow! Apparently we don't account for recursive dependencies.
			// || direct_dependencies
			// .iter()
			// .any(|intermediate_dependency| self.depends_on(*intermediate_dependency, b))
		} else {
			false
		}
	}

	fn cmp(&self, a: u32, b: u32) -> Option<cmp::Ordering> {
		if a == b {
			// Some(cmp::Ordering::Equal)
			unreachable!()
		} else if self.depends_on(b, a) {
			Some(cmp::Ordering::Less)
		} else if self.depends_on(a, b) {
			Some(cmp::Ordering::Greater)
		} else {
			// None
			unreachable!()
		}
	}

	pub fn allowed_before(&self, a: u32, b: u32) -> bool {
		if let Some(ord) = self.cmp(a, b) {
			// ord != cmp::Ordering::Greater
			ord == cmp::Ordering::Less
		} else {
			// true
			unreachable!()
		}
	}
}

fn parse(input: &str) -> (Ordering, Vec<Vec<u32>>) {
	let (top, bottom) = input
		.split_once("\n\n")
		.expect("Should have been able to split our input");
	let mut dependencies = HashMap::new();
	top.lines().for_each(|line| {
		let mut split = helpers::parse::into_unsigned_nums(line, "|");
		let dependency = split.next().expect("Expected first number");
		let page = split.next().expect("Expected second number");
		assert!(split.next().is_none(), "Third value in line");
		dependencies
			.entry(page)
			.or_insert_with(HashSet::new)
			.insert(dependency);
	});
	let updates = bottom
		.lines()
		.map(|line| helpers::parse::into_unsigned_nums(line, ",").collect())
		.collect();
	(Ordering(dependencies), updates)
}

fn middle_entry(list: &[u32]) -> u32 {
	list[list.len() / 2]
}

#[cfg(test)]
mod tests {
	use crate::{day05_print_queue::middle_entry, helpers};

	use super::parse;

	#[test]
	fn ex01() {
		let input = helpers::read::to_string("inputs/day05/ex01.txt");
		let (ord, updates) = parse(&input);
		let presorted: Vec<_> = updates
			.iter()
			.map(|update| update.is_sorted_by(|a, b| ord.allowed_before(*a, *b)))
			.collect();
		let expected_truths = vec![true, true, true, false, false, false];
		assert_eq!(expected_truths, presorted);
		let middles: Vec<_> = expected_truths
			.iter()
			.zip(updates)
			.filter_map(|(p, v)| p.then(|| middle_entry(&v)))
			.collect();
		let expected_middles = vec![61, 53, 29];
		assert_eq!(expected_middles, middles);
	}
}
