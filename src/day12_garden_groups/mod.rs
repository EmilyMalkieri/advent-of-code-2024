use std::collections::HashSet;

use crate::helpers::{
	self,
	types::{
		direction::Direction,
		grid::{Grid, Pos},
	},
};

#[allow(dead_code)]
pub fn solve_1() -> usize {
	let garden = Garden::try_from(helpers::read::to_lines("inputs/day12/input.txt"))
		.expect("Should have been a valid garden.");
	let grouped_into_regions = garden.group_by_adjacent_values(&Direction::clockwise_90deg());
	grouped_into_regions.values().map(cost_of_fencing).sum()
}

#[allow(dead_code)]
pub fn solve_2() -> usize {
	todo!()
}

type Garden = Grid<String>;
type Region = HashSet<Pos>;
fn area(region: &Region) -> usize {
	region.len()
}

fn circumference(region: &Region) -> usize {
	region
		.iter()
		.map(|plot| {
			Direction::clockwise_90deg()
				.iter()
				.filter(|dir| !region.contains(&plot.get_adjacent(**dir)))
				.count()
		})
		.sum()
}

fn cost_of_fencing(region: &Region) -> usize {
	area(region) * circumference(region)
}

#[cfg(test)]
mod tests {
	use crate::helpers;
	use crate::helpers::types::direction::Direction;

	use super::cost_of_fencing;
	use super::Garden;

	#[test]
	fn ex01() {
		let garden = Garden::try_from(helpers::read::to_lines("inputs/day12/ex01.txt"))
			.expect("Should have been a valid garden.");
		let grouped_into_regions = garden.group_by_adjacent_values(&Direction::clockwise_90deg());
		let expected = 140;
		let total_fencing_cost: usize = grouped_into_regions.values().map(cost_of_fencing).sum();
		assert_eq!(expected, total_fencing_cost);
	}

	#[test]
	fn ex02() {
		let garden = Garden::try_from(helpers::read::to_lines("inputs/day12/ex02.txt"))
			.expect("Should have been a valid garden.");
		let grouped_into_regions = garden.group_by_adjacent_values(&Direction::clockwise_90deg());
		let expected = 772;
		let total_fencing_cost: usize = grouped_into_regions.values().map(cost_of_fencing).sum();
		assert_eq!(expected, total_fencing_cost);
	}

	#[test]
	fn ex03() {
		let garden = Garden::try_from(helpers::read::to_lines("inputs/day12/ex03.txt"))
			.expect("Should have been a valid garden.");
		let grouped_into_regions = garden.group_by_adjacent_values(&Direction::clockwise_90deg());
		let expected = 1930;
		let total_fencing_cost: usize = grouped_into_regions.values().map(cost_of_fencing).sum();
		assert_eq!(expected, total_fencing_cost);
	}
}
