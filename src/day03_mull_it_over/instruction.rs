use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Instruction {
	mul(u32, u32),
}

static DAY03_REGEX: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Unable to compile regex."));

impl Instruction {
	pub fn execute(&self) -> Option<u32> {
		match self {
			Instruction::mul(a, b) => Some(a * b),
		}
	}

	fn from_capture(cap: regex::Captures) -> Option<Self> {
		Some(Instruction::mul(
			u32::from_str_radix(cap.get(1).expect("Regex failed").as_str(), 10).expect("Not a number"),
			u32::from_str_radix(cap.get(2).expect("Regex failed").as_str(), 10).expect("Not a number"),
		))
	}
	pub fn parse_many(blob: &str) -> Vec<Self> {
		DAY03_REGEX
			.captures_iter(blob)
			.filter_map(Instruction::from_capture)
			.collect()
	}
}
