use crate::helpers::read;
use crate::helpers::types::grid;

#[allow(dead_code)]
pub fn solve_1() -> u32 {
	let grid = grid::Grid::from(read::to_lines("inputs/day04/input.txt"));
	count_all_xmases(&grid)
}

#[allow(dead_code)]
pub fn solve_2() -> u32 {
	todo!()
}

/// Count number of "XMAS" strings starting at this position, which represents a X.
///
/// XMAS can be spelled forwards or backwards in diagonal, horizontal, or vertical position.
///
/// We assume that we've already verified that `Grid::get(pos)` is `Some("X")`.
fn count_xmases_at_x(grid: &grid::Grid<String>, pos: grid::Pos) -> u32 {
	let mut celebrations = 0;
	let needles = ["M", "A", "S"];
	'direction: for dir in grid::Direction::clockwise() {
		let mut prev_pos = pos;
		for needle in needles {
			let curr_pos = prev_pos.get_adjacent(&dir);
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

#[cfg(test)]
mod tests {

	use crate::day04_ceres_search::count_all_xmases;
	use crate::helpers::read;
	use crate::helpers::types::grid;

	#[test]
	fn ex01() {
		let grid = grid::Grid::from(read::to_lines("inputs/day04/ex01.txt"));
		let xmases: u32 = count_all_xmases(&grid);
		assert_eq!(4, xmases);
	}

	#[test]
	fn ex02() {
		let grid = grid::Grid::from(read::to_lines("inputs/day04/ex02.txt"));
		let xmases: u32 = count_all_xmases(&grid);
		assert_eq!(18, xmases);
	}

	#[test]
	fn ex02_but_with_dots() {
		let grid = grid::Grid::from(read::to_lines("inputs/day04/ex02_dots.txt"));
		let xmases: u32 = count_all_xmases(&grid);
		assert_eq!(18, xmases);
	}
}
