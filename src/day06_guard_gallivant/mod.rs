use core::str::FromStr;
use std::collections::HashSet;

use crate::helpers::{
	self,
	types::grid::{self},
};

#[allow(dead_code)]
pub fn solve_1() -> usize {
	let mut map =
		grid::Grid::<MapObject>::try_from(helpers::read::to_lines("inputs/day06/input.txt"))
			.expect("Should have been able to parse input");
	let mut guard = Walker::locate(&map).expect("Expected a guard.");
	let trail = trail_the_guard(&mut guard, &mut map);
	trail.into_iter().collect::<HashSet<_>>().len()
}

#[allow(dead_code)]
pub fn solve_2() -> usize {
	todo!()
}

/// Follow the guard as she leaves the map, reporting the route taken.
///
/// This includes her starting location but not her final location outside the map boundaries.
fn trail_the_guard(guard: &mut Walker, map: &mut Map) -> Vec<grid::Pos> {
	let mut trail = vec![guard.pos];

	while let action = guard.walk(map)
		&& action != Action::EscapeToFreedom
	{
		match action {
			Action::EscapeToFreedom => unreachable!(),
			Action::MoveAhead(pos) => trail.push(pos),
			Action::Turn(_) => (),
		}
	}

	trail
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapObject {
	FreeSpace,
	Guard,
	Obstruction,
}

impl FromStr for MapObject {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"." => Ok(MapObject::FreeSpace),
			"^" => Ok(MapObject::Guard),
			"#" => Ok(MapObject::Obstruction),
			_ => Err(format!("Not a valid map object: {s}")),
		}
	}
}

type Map = grid::Grid<MapObject>;
#[derive(Debug)]
struct Walker {
	pos: grid::Pos,
	dir: grid::Direction,
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
	EscapeToFreedom,
	MoveAhead(grid::Pos),
	Turn(grid::Direction),
}

impl Walker {
	fn locate(map: &Map) -> Option<Self> {
		map.into_iter()
			.find(|pos| Some(&MapObject::Guard) == map.get(pos))
			.map(|pos| Walker {
				pos,
				dir: grid::Direction::Up,
			})
	}

	/// Walk one step if possible and report the action taken.
	pub fn walk(&mut self, map: &mut Map) -> Action {
		let next_tile = self.pos.get_adjacent(self.dir);
		match map.get(&next_tile) {
			Some(&MapObject::Guard) => unreachable!(),
			Some(&MapObject::FreeSpace) => {
				map.swap(&self.pos, &next_tile);
				self.pos = next_tile;
				Action::MoveAhead(self.pos)
			}
			Some(&MapObject::Obstruction) => {
				self.dir = self.dir.turn_clockwise_90deg();
				Action::Turn(self.dir)
			}
			None => {
				map.replace(&self.pos, MapObject::FreeSpace);
				self.pos = next_tile;
				Action::EscapeToFreedom
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	use crate::helpers::{self, types::grid};

	use super::{trail_the_guard, MapObject, Walker};

	#[test]
	fn ex01() {
		let mut map =
			grid::Grid::<MapObject>::try_from(helpers::read::to_lines("inputs/day06/ex01.txt"))
				.expect("Should have been able to parse this grid.");
		let mut guard = Walker::locate(&map).expect("Should have found our guard!");
		let route = trail_the_guard(&mut guard, &mut map);
		let unique_count = route.into_iter().collect::<HashSet<_>>().len();
		assert_eq!(41, unique_count);
	}
}
