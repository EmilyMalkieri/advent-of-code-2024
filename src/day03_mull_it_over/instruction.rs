use std::sync::LazyLock;

use regex::Regex;

use crate::helpers;

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
	Regex::new(
		//cspell: disable-next-line
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
				cap.name("mul_a")
					.expect("Regex failed")
					.as_str()
					.parse::<u32>()
					.expect("Not a number"),
				cap.name("mul_b")
					.expect("Regex failed")
					.as_str()
					.parse::<u32>()
					.expect("Not a number"),
			))
		} else {
			None
		}
	}

	pub fn parse_many_naively(blob: &str) -> impl Iterator<Item = Self> + use<'_> {
		helpers::parse::through_regex(blob, &REGEX, Instruction::from_capture)
	}

	pub fn parse_many(blob: &str) -> impl Iterator<Item = Self> + use<'_> {
		let mut parser = Parser::Enabled;
		Self::parse_many_naively(blob).filter_map(move |instruction| match instruction {
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
					Some(Instruction::mul(a, b))
				} else {
					None
				}
			}
		})
	}
}
