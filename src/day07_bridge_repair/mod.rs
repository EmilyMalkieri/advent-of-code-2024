use std::collections::HashSet;

use crate::helpers;

type Num = u64;

#[allow(dead_code)]
pub fn solve_1() -> Num {
	let lines = helpers::parse_each_line::by(
		helpers::read::to_lines("inputs/day07/input.txt"),
		Equation::from,
	);
	let ops = HashSet::from_iter([Operator::Add, Operator::Multiply]);
	lines
		.filter(|eq| eq.is_solvable(&ops.clone()))
		.map(|eq| eq.result)
		.sum()
}

#[allow(dead_code)]
pub fn solve_2() -> Num {
	let lines = helpers::parse_each_line::by(
		helpers::read::to_lines("inputs/day07/input.txt"),
		Equation::from,
	);
	let ops = HashSet::from_iter([Operator::Add, Operator::Multiply, Operator::Concatenate]);
	lines
		.filter(|eq| eq.is_solvable(&ops.clone()))
		.map(|eq| eq.result)
		.sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Operator {
	Add,
	Multiply,
	Concatenate,
}

impl Operator {
	pub fn calc(self, a: Num, b: Num) -> Num {
		match self {
			Operator::Add => a + b,
			Operator::Multiply => a * b,
			Operator::Concatenate => {
				let mut cc = a.to_string();
				cc.push_str(&b.to_string());
				cc.parse::<Num>()
					.expect("Unable to parse concatenated number")
			}
		}
	}
}

#[derive(Debug)]
struct Equation {
	result: Num,
	nums: Vec<Num>,
}

impl Equation {
	pub fn is_solvable(&self, allowed_operators: &HashSet<Operator>) -> bool {
		match self.nums.len() {
			0 => false,
			1 => Some(&self.result) == self.nums.first(),
			len => helpers::iter::permutations(&allowed_operators.iter(), len - 1).any(|p| {
				let mut ops = p.iter();
				let nums = self.nums.clone().into_iter();
				Some(self.result)
					== nums.reduce(|acc, curr| {
						ops.next()
							.expect("We literally created this with the correct number of elements")
							.calc(acc, curr)
					})
			}),
		}
	}
}

impl From<String> for Equation {
	fn from(value: String) -> Self {
		let (res, eq) = value
			.split_once(':')
			.expect("Should have been able to split this line into result and equation");
		let result = res.parse::<Num>().expect("Result should be a number.");
		let nums = helpers::parse::into_unsigned_nums_by_whitespace(eq).collect();
		Equation { result, nums }
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	use super::Equation;
	use super::Num;
	use crate::{day07_bridge_repair::Operator, helpers};

	#[test]
	fn ex01() {
		let lines = helpers::parse_each_line::by(
			helpers::read::to_lines("inputs/day07/ex01.txt"),
			Equation::from,
		);
		let ops = HashSet::from_iter([Operator::Add, Operator::Multiply]);
		let sum = lines
			.filter(|eq| eq.is_solvable(&ops.clone()))
			.map(|eq| eq.result)
			.sum();
		let expected: Num = 3749;
		assert_eq!(expected, sum);
	}

	#[test]
	fn ex01_concat() {
		let lines = helpers::parse_each_line::by(
			helpers::read::to_lines("inputs/day07/ex01.txt"),
			Equation::from,
		);
		let ops = HashSet::from_iter([Operator::Add, Operator::Multiply, Operator::Concatenate]);
		let sum = lines
			.filter(|eq| eq.is_solvable(&ops.clone()))
			.map(|eq| eq.result)
			.sum();
		let expected: Num = 11387;
		assert_eq!(expected, sum);
	}
}
