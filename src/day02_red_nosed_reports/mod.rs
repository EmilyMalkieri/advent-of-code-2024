use report::Report;

use crate::helpers;

mod report;
mod trend;

#[allow(dead_code)]
pub fn solve_1() -> usize {
	let reports = helpers::read::and_parse_each_line_into_type("inputs/day02/input.txt");
	reports.filter(|r: &Report| r.is_safe()).count()
}

#[allow(dead_code)]
pub fn solve_2() -> usize {
	let reports = helpers::read::and_parse_each_line_into_type("inputs/day02/input.txt");
	reports
		.filter(|r: &Report| r.is_safe_with_dampener())
		.count()
}

#[cfg(test)]
mod test {
	use crate::helpers;

	use super::report::Report;

	#[test]
	fn ex01() {
		let reports = helpers::read::and_parse_each_line_into_type("inputs/day02/ex01.txt");
		let safety: Vec<_> = reports.map(|r: Report| r.is_safe()).collect();
		assert_eq!(safety, vec![true, false, false, false, false, true]);
		assert_eq!(safety.iter().filter(|&&e| e).count(), 2);
	}

	#[test]
	fn ex02() {
		let reports = helpers::read::and_parse_each_line_into_type("inputs/day02/ex01.txt");
		let safety: Vec<_> = reports.map(|r: Report| r.is_safe_with_dampener()).collect();
		assert_eq!(safety, vec![true, false, false, true, true, true]);
		assert_eq!(safety.iter().filter(|&&e| e).count(), 4);
	}
}
