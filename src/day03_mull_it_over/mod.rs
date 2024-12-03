mod instruction;

#[cfg(test)]
mod tests {
	use crate::{day03_mull_it_over::instruction::Instruction, helpers};

	#[test]
	fn ex01() {
		let memory = helpers::read::to_string("inputs/day03/ex01.txt");
		let instructions = Instruction::parse_many(&memory);
		assert!(instructions.is_some());
		let instructions = instructions.unwrap();
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
