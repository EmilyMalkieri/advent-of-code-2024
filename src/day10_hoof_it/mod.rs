mod height;
use std::collections::HashSet;

use height::Height;

use crate::helpers::{
	self,
	types::{
		direction::Direction,
		grid::{self, Pos},
	},
};

#[allow(dead_code)]
pub fn solve_1() -> usize {
	let map = Map::try_from(helpers::read::to_lines("inputs/day10/input.txt"))
		.expect("Should have been able to construct our map.");
	let trailheads: Vec<Trailhead> = map
		.into_iter()
		.filter_map(|pos| calculate_trailhead(pos, &map))
		.collect();
	trailheads.iter().map(|th| th.peaks).sum()
}

#[allow(dead_code)]
pub fn solve_2() -> usize {
	let map = Map::try_from(helpers::read::to_lines("inputs/day10/input.txt"))
		.expect("Should have been able to construct our map.");
	map.into_iter()
		.filter_map(|pos| calculate_trailhead(pos, &map))
		.map(|th| th.trails.len())
		.sum()
}

type Map = grid::Grid<Height>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Trailhead {
	peaks: usize,
	trails: Vec<Vec<Pos>>,
}

fn could_be_trailhead(pos: Pos, map: &Map) -> bool {
	Some(&Height::Height(0)) == map.get(&pos)
}

fn calculate_trailhead(pos: Pos, map: &Map) -> Option<Trailhead> {
	if !could_be_trailhead(pos, map) {
		return None;
	}
	let trails: Vec<_> = Direction::clockwise_90deg()
		.iter()
		.filter_map(|dir| {
			let next_trail_section = pos.get_adjacent(*dir);
			if let Some(&Height::Height(1)) = map.get(&next_trail_section) {
				Some((next_trail_section, dir))
			} else {
				None
			}
		})
		.flat_map(|(next_trail_section, dir)| {
			find_trails(next_trail_section, map, dir.reverse(), vec![pos])
		})
		.collect();
	let peaks = trails
		.iter()
		.filter_map(|trail| trail.last().copied())
		.collect::<HashSet<_>>()
		.len();
	if peaks > 0 {
		Some(Trailhead { peaks, trails })
	} else {
		None
	}
}

fn find_trails(pos: Pos, map: &Map, from: Direction, the_trail_so_far: Vec<Pos>) -> Vec<Vec<Pos>> {
	let Some(&Height::Height(height)) = map.get(&pos) else {
		unreachable!("We checked this in advance")
	};
	if height == 9 {
		return vec![the_trail_so_far];
	}

	Direction::clockwise_90deg()
		.iter()
		.filter_map(|dir| {
			if *dir == from {
				return None;
			}
			let next_trail_section = pos.get_adjacent(*dir);
			if let Some(&Height::Height(next_height)) = map.get(&next_trail_section)
				&& next_height == height + 1
			{
				Some((next_trail_section, dir))
			} else {
				None
			}
		})
		.flat_map(|(next_trail_section, dir)| {
			let mut trail = the_trail_so_far.clone();
			trail.push(next_trail_section);
			find_trails(next_trail_section, map, dir.reverse(), trail)
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::{calculate_trailhead, Map, Trailhead};
	use crate::helpers;

	#[test]
	fn ex01() {
		let map = Map::try_from(helpers::read::to_lines("inputs/day10/ex01.txt"))
			.expect("Should have been able to construct our map.");
		let peaks: Vec<_> = map
			.into_iter()
			.filter_map(|pos| calculate_trailhead(pos, &map))
			.map(|th| th.peaks)
			.collect();
		assert_eq!(vec![1], peaks);
	}

	#[test]
	fn ex02() {
		let map = Map::try_from(helpers::read::to_lines("inputs/day10/ex02.txt"))
			.expect("Should have been able to construct our map.");
		let peaks: Vec<_> = map
			.into_iter()
			.filter_map(|pos| calculate_trailhead(pos, &map))
			.map(|th| th.peaks)
			.collect();
		assert_eq!(vec![2], peaks);
	}

	#[test]
	fn ex03() {
		let map = Map::try_from(helpers::read::to_lines("inputs/day10/ex03.txt"))
			.expect("Should have been able to construct our map.");
		let peaks: Vec<_> = map
			.into_iter()
			.filter_map(|pos| calculate_trailhead(pos, &map))
			.map(|th| th.peaks)
			.collect();
		assert_eq!(vec![4], peaks);
	}

	#[test]
	fn ex04() {
		let map = Map::try_from(helpers::read::to_lines("inputs/day10/ex04.txt"))
			.expect("Should have been able to construct our map.");
		let peaks: Vec<_> = map
			.into_iter()
			.filter_map(|pos| calculate_trailhead(pos, &map))
			.map(|th| th.peaks)
			.collect();
		assert_eq!(vec![1, 2], peaks);
	}

	#[test]
	fn ex05() {
		let map = Map::try_from(helpers::read::to_lines("inputs/day10/ex05.txt"))
			.expect("Should have been able to construct our map.");
		let peaks: Vec<_> = map
			.into_iter()
			.filter_map(|pos| calculate_trailhead(pos, &map))
			.map(|th| th.peaks)
			.collect();
		assert_eq!(vec![5, 6, 5, 3, 1, 3, 5, 3, 5], peaks);
	}

	#[test]
	fn ex06() {
		let map = Map::try_from(helpers::read::to_lines("inputs/day10/ex06.txt"))
			.expect("Should have been able to construct our map.");
		let trails: Vec<_> = map
			.into_iter()
			.filter_map(|pos| calculate_trailhead(pos, &map))
			.map(|th| th.trails.len())
			.collect();
		assert_eq!(vec![3], trails);
	}

	#[test]
	fn ex07() {
		let map = Map::try_from(helpers::read::to_lines("inputs/day10/ex07.txt"))
			.expect("Should have been able to construct our map.");
		let trails: Vec<_> = map
			.into_iter()
			.filter_map(|pos| calculate_trailhead(pos, &map))
			.map(|th| th.trails.len())
			.collect();
		assert_eq!(vec![13], trails);
	}

	#[test]
	fn ex08() {
		let map = Map::try_from(helpers::read::to_lines("inputs/day10/ex08.txt"))
			.expect("Should have been able to construct our map.");
		let peaks: Vec<_> = map
			.into_iter()
			.filter_map(|pos| calculate_trailhead(pos, &map))
			.map(|th| th.trails.len())
			.collect();
		assert_eq!(vec![227], peaks);
	}

	#[test]
	fn ex05_trails() {
		let map = Map::try_from(helpers::read::to_lines("inputs/day10/ex05.txt"))
			.expect("Should have been able to construct our map.");
		let trails: Vec<_> = map
			.into_iter()
			.filter_map(|pos| calculate_trailhead(pos, &map))
			.map(|th| th.trails.len())
			.collect();
		assert_eq!(vec![20, 24, 10, 4, 1, 4, 5, 8, 5], trails);
	}
}
