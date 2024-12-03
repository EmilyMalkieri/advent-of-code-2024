use instruction::Instruction;

use crate::helpers;

mod instruction;

#[allow(dead_code)]
pub fn solve_1() -> u32 {
	let memory = helpers::read::to_string("inputs/day03/input.txt");
	let instructions = Instruction::parse_many(&memory);
	instructions
		.iter()
		.map(|instruction| instruction.execute().expect("Not mul() apparently"))
		.sum()
}

#[allow(dead_code)]
pub fn solve_2() -> usize {
	todo!()
}

#[cfg(test)]
mod tests {
	use crate::{day03_mull_it_over::instruction::Instruction, helpers};

	#[test]
	fn ex01() {
		let memory = helpers::read::to_string("inputs/day03/ex01.txt");
		let instructions = Instruction::parse_many(&memory);
		let expected = vec![
			Instruction::mul(2, 4),
			Instruction::mul(5, 5),
			Instruction::mul(11, 8),
			Instruction::mul(8, 5),
		];
		assert_eq!(instructions, expected);
		assert_eq!(
			instructions
				.iter()
				.map(|instruction| instruction
					.execute()
					.expect("Must have gotten the wrong instruction type here"))
				.sum::<u32>(),
			161
		)
	}
}
