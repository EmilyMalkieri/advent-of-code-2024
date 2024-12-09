use core::cmp::max;
use core::str::FromStr;

use itertools::Itertools as _;

use crate::helpers;

type Num = u64;

#[allow(dead_code)]
pub fn solve_1() -> Num {
	let mut disk = expand(&parse(&helpers::read::to_string("inputs/day09/input.txt")));
	fragment(&mut disk);
	checksum(&disk)
}

#[allow(dead_code)]
pub fn solve_2() -> Num {
	let mut disk = parse(&helpers::read::to_string("inputs/day09/input.txt"));
	move_without_fragmenting(&mut disk);
	checksum(&expand(&disk))
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

fn expand(list: &[(DiskEntry, usize)]) -> Vec<DiskEntry> {
	#[allow(clippy::pattern_type_mismatch)]
	list
		.iter()
		.flat_map(|(entry, count)| [*entry].repeat(*count))
		.collect()
}

fn parse(input: &str) -> Vec<(DiskEntry, usize)> {
	let mut alternating = [|id: Num| DiskEntry::FilePart(id), |_: Num| DiskEntry::Free]
		.iter()
		.cycle();
	helpers::parse_each_char::into_digits(input)
		.iter()
		.enumerate()
		.map(|(idx, digit)| {
			let id = (idx / 2).try_into().expect("Index should fit into u32");
			let count = (*digit).into();
			(
				alternating
					.next()
					.expect("This iter literally cycles it should have an element")(id),
				count,
			)
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

fn move_without_fragmenting(disk: &mut Vec<(DiskEntry, usize)>) {
	let mut back_idx = max(0, disk.len() - 1);
	#[allow(clippy::pattern_type_mismatch)]
	while 0 < back_idx {
		while back_idx > 0
			&& let Some((DiskEntry::Free, _)) = disk.get(back_idx)
		{
			back_idx -= 1;
		}
		if let Some((DiskEntry::FilePart(id), width)) = disk.get(back_idx) {
			if let Some((front_idx, (_, space_available))) = disk
				.iter()
				.find_position(|(kind, size)| *kind == DiskEntry::Free && size >= width)
				&& front_idx < back_idx
			{
				if space_available == width {
					disk.swap(front_idx, back_idx);
					back_idx -= 1;
				} else {
					disk
						.get(front_idx)
						.replace(&(DiskEntry::FilePart(*id), *width));
					disk.get(back_idx).replace(&(DiskEntry::Free, *width));
					disk.insert(front_idx + 1, (DiskEntry::Free, space_available - width));
					// we don't decrement back_idx here because we inserted an element, so effectively we've decremented it
				}
			}
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

	use crate::helpers;

	use super::{checksum, expand, fragment, move_without_fragmenting, parse, DiskEntry};

	#[test]
	fn ex01() {
		let mut disk = expand(&parse(&helpers::read::to_string("inputs/day09/ex01.txt")));
		fragment(&mut disk);
		assert_eq!(1928, checksum(&disk));
	}

	#[test]
	fn ex02_parse_only() {
		let actual = expand(&parse(&helpers::read::to_string("inputs/day09/ex02.txt")));
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

	#[test]
	fn ex01_without_fragmenting() {
		let mut disk = parse(&helpers::read::to_string("inputs/day09/ex01.txt"));
		move_without_fragmenting(&mut disk);
		assert_eq!(2858, checksum(&expand(&disk)));
	}
}
