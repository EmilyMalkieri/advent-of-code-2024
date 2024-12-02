use report::Report;

use crate::helpers;

mod report;
mod trend;

pub fn solve_1() -> usize {
	let reports: Vec<Report> =
		helpers::read_and_parse_each_lines_nums_into("inputs/day02/input.txt");
	reports.iter().filter(|r| r.is_safe()).count()
}

pub fn solve_2() -> usize {
	let reports: Vec<Report> =
		helpers::read_and_parse_each_lines_nums_into("inputs/day02/input.txt");
	reports.iter().filter(|r| r.is_safe_with_dampener()).count()
}

#[cfg(test)]
mod test {
	use crate::helpers;

	use super::report::Report;

	#[test]
	fn ex01() {
		let reports: Vec<Report> =
			helpers::read_and_parse_each_lines_nums_into("inputs/day02/ex01.txt");
		let safety: Vec<_> = reports.iter().map(|r| r.is_safe()).collect();
		assert_eq!(safety, vec![true, false, false, false, false, true]);
		assert_eq!(safety.iter().filter(|&&e| e).count(), 2);
	}

	#[test]
	fn ex02() {
		let reports: Vec<Report> =
			helpers::read_and_parse_each_lines_nums_into("inputs/day02/ex01.txt");
		let safety: Vec<_> = reports.iter().map(|r| r.is_safe_with_dampener()).collect();
		assert_eq!(safety, vec![true, false, false, true, true, true]);
		assert_eq!(safety.iter().filter(|&&e| e).count(), 4);
	}
}
