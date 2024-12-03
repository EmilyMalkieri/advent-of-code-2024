use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Instruction {
	mul(u32, u32),
	r#do,
	//cspell: disable-next-line
	dont,
}

#[derive(Debug, PartialEq, Eq)]
enum Parser {
	Enabled,
	Disabled,
}

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
	//cspell: disable-next-line
	Regex::new(
		r"(?<do>do)\(\)|(?<dont>don't)\(\)|(?<mul>mul)\((?<mul_a>\d{1,3}),(?<mul_b>\d{1,3})\)",
	)
	.expect("Unable to compile regex.")
});

impl Instruction {
	pub fn execute(&self) -> Option<u32> {
		match self {
			Instruction::mul(a, b) => Some(a * b),
			Instruction::r#do => None,
			//cspell: disable-next-line
			Instruction::dont => None,
		}
	}

	fn from_capture(cap: regex::Captures) -> Option<Self> {
		if cap.name("do").is_some() {
			Some(Instruction::r#do)
		}
		// cspell: disable-next-line
		else if cap.name("dont").is_some() {
			//cspell: disable-next-line
			Some(Instruction::dont)
		} else if cap.name("mul").is_some() {
			Some(Instruction::mul(
				u32::from_str_radix(cap.name("mul_a").expect("Regex failed").as_str(), 10)
					.expect("Not a number"),
				u32::from_str_radix(cap.name("mul_b").expect("Regex failed").as_str(), 10)
					.expect("Not a number"),
			))
		} else {
			None
		}
	}
	pub fn parse_many_naively(blob: &str) -> Vec<Self> {
		REGEX
			.captures_iter(blob)
			.filter_map(Instruction::from_capture)
			.collect()
	}

	pub fn parse_many(blob: &str) -> Vec<Self> {
		let mut parser = Parser::Enabled;
		Self::parse_many_naively(blob)
			.iter()
			.filter_map(|instruction| match instruction {
				Instruction::r#do => {
					parser = Parser::Enabled;
					Some(Instruction::r#do)
				}
				//cspell: disable-next-line
				Instruction::dont => {
					parser = Parser::Disabled;
					//cspell: disable-next-line
					Some(Instruction::dont)
				}
				Instruction::mul(a, b) => {
					if parser == Parser::Enabled {
						Some(Instruction::mul(*a, *b))
					} else {
						None
					}
				}
			})
			.collect()
	}
}
