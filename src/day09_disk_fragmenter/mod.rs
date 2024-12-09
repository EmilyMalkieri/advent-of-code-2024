use core::cmp::max;
use core::str::FromStr;

use crate::helpers;

type Num = u64;

#[allow(dead_code)]
pub fn solve_1() -> Num {
	let mut disk = parse(&helpers::read::to_string("inputs/day09/input.txt"));
	fragment(&mut disk);
	checksum(&disk)
}

#[allow(dead_code)]
pub fn solve_2() -> usize {
	todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DiskEntry {
	Free,
	FilePart(Num),
}

impl FromStr for DiskEntry {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"." => Ok(DiskEntry::Free),
			"0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => Ok(DiskEntry::FilePart(
				s.parse::<Num>().expect("We can parse one digit"),
			)),
			other => Err(format!("Not a digit or a free space: {other}")),
		}
	}
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

fn fragment(disk: &mut [DiskEntry]) {
	let mut front_idx = 0;
	let mut back_idx = max(0, disk.len() - 1);
	while front_idx < back_idx {
		while front_idx < back_idx && Some(&DiskEntry::Free) != disk.get(front_idx) {
			front_idx += 1;
		}
		while back_idx > front_idx && Some(&DiskEntry::Free) == disk.get(back_idx) {
			back_idx -= 1;
		}
		if front_idx < back_idx {
			disk.swap(front_idx, back_idx);
		}
	}
}

fn checksum(disk: &[DiskEntry]) -> Num {
	disk
		.iter()
		.enumerate()
		.filter_map(|(idx, entry)| {
			if let &DiskEntry::FilePart(id) = entry {
				let mul: Num = idx.try_into().expect("Can't fit index");
				Some(id * mul)
			} else {
				None
			}
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use core::str::FromStr as _;

	use crate::{day09_disk_fragmenter::fragment, helpers};

	use super::{checksum, parse, DiskEntry};

	#[test]
	fn ex01() {
		let mut disk = parse(&helpers::read::to_string("inputs/day09/ex01.txt"));
		fragment(&mut disk);
		assert_eq!(1928, checksum(&disk));
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

	#[test]
	fn ex01_checksum_only() {
		let disk = helpers::parse_each_char::non_indexed(
			&helpers::read::to_string("inputs/day09/checksum.txt"),
			DiskEntry::from_str,
		)
		.into_iter()
		.collect::<Result<Vec<_>, _>>()
		.expect("Should have been able to read testcase input.");
		let expected = 1928;
		assert_eq!(expected, checksum(&disk));
	}
}
