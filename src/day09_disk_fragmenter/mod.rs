use crate::helpers;

#[allow(dead_code)]
pub fn solve_1() -> usize {
	todo!()
}

#[allow(dead_code)]
pub fn solve_2() -> usize {
	todo!()
}

type Num = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DiskEntry {
	Free,
	FilePart(Num),
}

fn parse(input: &str) -> Vec<DiskEntry> {
	let mut alternating = [|id: Num| DiskEntry::FilePart(id), |_: Num| DiskEntry::Free]
		.iter()
		.cycle();
	helpers::parse_each_char::into_digits(input)
		.iter()
		.enumerate()
		.flat_map(|(idx, digit)| {
			let id = (idx / 2).try_into().expect("Index should fit into u32");
			let count = (*digit).into();
			[alternating.next().expect(
				"This iter literally cycles it should have an element",
			)(id)]
			.repeat(count)
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use crate::helpers;

	use super::{parse, DiskEntry};

	#[test]
	fn ex01() {
		todo!()
	}

	#[test]
	fn ex02() {
		let actual = parse(&helpers::read::to_string("inputs/day09/ex02.txt"));
		let expected = vec![
			DiskEntry::FilePart(0),
			DiskEntry::Free,
			DiskEntry::Free,
			DiskEntry::FilePart(1),
			DiskEntry::FilePart(1),
			DiskEntry::FilePart(1),
			DiskEntry::Free,
			DiskEntry::Free,
			DiskEntry::Free,
			DiskEntry::Free,
			DiskEntry::FilePart(2),
			DiskEntry::FilePart(2),
			DiskEntry::FilePart(2),
			DiskEntry::FilePart(2),
			DiskEntry::FilePart(2),
		];
		assert_eq!(actual, expected);
	}
}
