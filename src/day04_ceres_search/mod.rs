use crate::helpers::read;
use crate::helpers::types::direction;
use crate::helpers::types::grid;

#[allow(dead_code)]
pub fn solve_1() -> u32 {
	let grid = grid::Grid::try_from(read::to_lines("inputs/day04/input.txt"))
		.expect("Should have been able to construct this grid.");
	count_all_xmases(&grid)
}

#[allow(dead_code)]
pub fn solve_2() -> u32 {
	let grid = grid::Grid::try_from(read::to_lines("inputs/day04/input.txt"))
		.expect("Should have been able to construct this grid.");
	count_all_crossed_masses(&grid)
}

/// Count number of "XMAS" strings starting at this position, which represents the X.
///
/// XMAS can be spelled forwards or backwards in diagonal, horizontal, or vertical position.
///
/// We assume that we've already verified that `Grid::get(pos)` is `Some("X")`.
fn count_xmases_at_x(grid: &grid::Grid<String>, pos: grid::Pos) -> u32 {
	let mut celebrations = 0;
	let needles = ["M", "A", "S"];
	'direction: for dir in direction::Direction::clockwise() {
		let mut prev_pos = pos;
		for needle in needles {
			let curr_pos = prev_pos.get_adjacent(dir);
			if let Some(value) = grid.get(&curr_pos)
				&& value == needle
			{
				prev_pos = curr_pos;
			} else {
				continue 'direction;
			}
		}
		celebrations += 1;
	}

	celebrations
}

fn count_all_xmases(grid: &grid::Grid<String>) -> u32 {
	grid
		.into_iter()
		.filter_map(|pos| {
			if let Some("X") = grid.get(&pos).map(String::as_str) {
				Some(count_xmases_at_x(grid, pos))
			} else {
				None
			}
		})
		.sum()
}

/// Count number of crossed "MAS" strings centering at this position, which represents the A.
///
/// Each individual MAS can be spelled forwards or backwards.
///
/// We assume that we've already verified that `Grid::get(pos)` is `Some("A")`.
fn is_crossed_mas_at_a(grid: &grid::Grid<String>, pos: grid::Pos) -> bool {
	let diag_falling = (
		direction::Direction::UpLeft,
		direction::Direction::DownRight,
	);
	let diag_rising = (
		direction::Direction::DownLeft,
		direction::Direction::UpRight,
	);

	[diag_falling, diag_rising].iter().all(|current_diag| {
		let (before, after) = (
			grid
				.get(&pos.get_adjacent(current_diag.0))
				.map(String::as_str),
			grid
				.get(&pos.get_adjacent(current_diag.1))
				.map(String::as_str),
		);
		matches!(
			(before, after),
			(Some("M"), Some("S")) | (Some("S"), Some("M"))
		)
	})
}

fn count_all_crossed_masses(grid: &grid::Grid<String>) -> u32 {
	u32::try_from(
		grid
			.into_iter()
			.filter(|pos| {
				if let Some("A") = grid.get(pos).map(String::as_str) {
					is_crossed_mas_at_a(grid, *pos)
				} else {
					false
				}
			})
			.count(),
	)
	.expect("Couldn't convert usize into u32")
}

#[cfg(test)]
mod tests {

	use crate::day04_ceres_search::{count_all_crossed_masses, count_all_xmases};
	use crate::helpers::read;
	use crate::helpers::types::grid;

	#[test]
	fn ex01() {
		let grid = grid::Grid::try_from(read::to_lines("inputs/day04/ex01.txt"))
			.expect("Should have been able to construct this grid.");
		let xmases: u32 = count_all_xmases(&grid);
		assert_eq!(4, xmases);
	}

	#[test]
	fn ex02() {
		let grid = grid::Grid::try_from(read::to_lines("inputs/day04/ex02.txt"))
			.expect("Should have been able to construct this grid.");
		let xmases: u32 = count_all_xmases(&grid);
		assert_eq!(18, xmases);
	}

	#[test]
	fn ex02_but_with_dots() {
		let grid = grid::Grid::try_from(read::to_lines("inputs/day04/ex02_dots.txt"))
			.expect("Should have been able to construct this grid.");
		let xmases: u32 = count_all_xmases(&grid);
		assert_eq!(18, xmases);
	}

	#[test]
	fn ex03() {
		let grid = grid::Grid::try_from(read::to_lines("inputs/day04/ex03.txt"))
			.expect("Should have been able to construct this grid.");
		let xmases: u32 = count_all_crossed_masses(&grid);
		assert_eq!(1, xmases);
	}

	#[test]
	fn ex04() {
		let grid = grid::Grid::try_from(read::to_lines("inputs/day04/ex02.txt"))
			.expect("Should have been able to construct this grid.");
		let xmases: u32 = count_all_crossed_masses(&grid);
		assert_eq!(9, xmases);
	}

	#[test]
	fn ex04_but_with_dots() {
		let grid = grid::Grid::try_from(read::to_lines("inputs/day04/ex04.txt"))
			.expect("Should have been able to construct this grid.");
		let xmases: u32 = count_all_crossed_masses(&grid);
		assert_eq!(9, xmases);
	}
}
