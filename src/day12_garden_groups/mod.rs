use std::collections::HashSet;

use itertools::Itertools as _;

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
	let garden = Garden::try_from(helpers::read::to_lines("inputs/day12/input.txt"))
		.expect("Should have been a valid garden.");
	let grouped_into_regions = garden.group_by_adjacent_values(&Direction::clockwise_90deg());
	grouped_into_regions
		.values()
		.map(cost_of_highly_discounted_fencing)
		.sum()
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

fn sides(region: &Region) -> usize {
	let potential_borders: HashSet<(Pos, Direction)> = region
		.iter()
		.flat_map(|plot| {
			Direction::clockwise_90deg()
				.iter()
				.filter(|dir| !region.contains(&plot.get_adjacent(**dir)))
				.map(|dir| (*plot, *dir))
				.collect_vec()
		})
		.collect();
	potential_borders
		.iter()
		.filter(|&&(plot, dir)| {
			let other = match dir {
				Direction::Left | Direction::Right => plot.get_adjacent(Direction::Down),
				Direction::Up | Direction::Down => plot.get_adjacent(Direction::Right),
				_ => unimplemented!(),
			};
			!potential_borders.contains(&(other, dir))
		})
		.count()
}

fn cost_of_highly_discounted_fencing(region: &Region) -> usize {
	area(region) * sides(region)
}

#[cfg(test)]
mod tests {
	use crate::day12_garden_groups::cost_of_highly_discounted_fencing;
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
	fn ex01_bulk_discounted() {
		let garden = Garden::try_from(helpers::read::to_lines("inputs/day12/ex01.txt"))
			.expect("Should have been a valid garden.");
		let grouped_into_regions = garden.group_by_adjacent_values(&Direction::clockwise_90deg());
		let expected = 80;
		let total_fencing_cost: usize = grouped_into_regions
			.values()
			.map(cost_of_highly_discounted_fencing)
			.sum();
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

	#[test]
	fn ex03_bulk_discounted() {
		let garden = Garden::try_from(helpers::read::to_lines("inputs/day12/ex03.txt"))
			.expect("Should have been a valid garden.");
		let grouped_into_regions = garden.group_by_adjacent_values(&Direction::clockwise_90deg());
		let expected = 1206;
		let total_fencing_cost: usize = grouped_into_regions
			.values()
			.map(cost_of_highly_discounted_fencing)
			.sum();
		assert_eq!(expected, total_fencing_cost);
	}

	#[test]
	fn ex04() {
		let garden = Garden::try_from(helpers::read::to_lines("inputs/day12/ex04.txt"))
			.expect("Should have been a valid garden.");
		let grouped_into_regions = garden.group_by_adjacent_values(&Direction::clockwise_90deg());
		let expected = 236;
		let total_fencing_cost: usize = grouped_into_regions
			.values()
			.map(cost_of_highly_discounted_fencing)
			.sum();
		assert_eq!(expected, total_fencing_cost);
	}

	#[test]
	fn ex05() {
		let garden = Garden::try_from(helpers::read::to_lines("inputs/day12/ex05.txt"))
			.expect("Should have been a valid garden.");
		let grouped_into_regions = garden.group_by_adjacent_values(&Direction::clockwise_90deg());
		let expected = 368;
		let total_fencing_cost: usize = grouped_into_regions
			.values()
			.map(cost_of_highly_discounted_fencing)
			.sum();
		assert_eq!(expected, total_fencing_cost);
	}
}
