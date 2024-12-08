use std::collections::{HashMap, HashSet};

use itertools::Itertools as _;

use crate::helpers::{self, types::point};

type Num = i32;
type Point = point::Point<Num>;

#[allow(dead_code)]
pub fn solve_1() -> usize {
	let input = helpers::read::to_string("inputs/day08/input.txt");
	let bounds = bounds(&input);
	let antennas_by_frequency =
		group_by_frequency(helpers::parse_each_char::zero_indexed(&input, try_parse));
	let unique_interference_points = antennas_by_frequency
		.into_values()
		.flat_map(|antennas| interference_points_at_distance_one(antennas, bounds))
		.collect::<HashSet<_>>();
	unique_interference_points.len()
}

#[allow(dead_code)]
pub fn solve_2() -> usize {
	let input = helpers::read::to_string("inputs/day08/input.txt");
	let bounds = bounds(&input);
	let antennas_by_frequency =
		group_by_frequency(helpers::parse_each_char::zero_indexed(&input, try_parse));
	let unique_interference_points = antennas_by_frequency
		.into_values()
		.flat_map(|antennas| interference_points(antennas, bounds))
		.collect::<HashSet<_>>();
	unique_interference_points.len()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Antenna {
	frequency: String,
	pos: Point,
}

impl Antenna {
	pub fn interference_points_at_distance_one(
		&self,
		other: &Self,
		bounds: (Point, Point),
	) -> Option<Vec<Point>> {
		if self.frequency == other.frequency {
			let mut points = Vec::new();
			let (a, b) = (
				other.pos.travel(&self.pos.distance_to(&other.pos)),
				self.pos.travel(&other.pos.distance_to(&self.pos)),
			);
			if a.is_bounded_by(&bounds.0, &bounds.1) {
				points.push(a);
			}
			if b.is_bounded_by(&bounds.0, &bounds.1) {
				points.push(b);
			}

			Some(points)
		} else {
			None
		}
	}

	pub fn interference_points(&self, other: &Self, bounds: (Point, Point)) -> Option<Vec<Point>> {
		if self.frequency != other.frequency {
			return None;
		}
		let mut points = Vec::new();
		let dis_from_here = self.pos.distance_to(&other.pos);
		for step in 0.. {
			let point = other.pos.travel(&(dis_from_here * step));
			if point.is_bounded_by(&bounds.0, &bounds.1) {
				points.push(point);
			} else {
				break;
			}
		}
		let dis_from_other = other.pos.distance_to(&self.pos);
		for step in 0.. {
			let point = self.pos.travel(&(dis_from_other * step));
			if point.is_bounded_by(&bounds.0, &bounds.1) {
				points.push(point);
			} else {
				break;
			}
		}

		Some(points)
	}
}

fn try_parse(s: &str, y: usize, x: usize) -> Option<Antenna> {
	if s == "." || s == "#" {
		None
	} else {
		Some(Antenna {
			frequency: String::from(s),
			pos: point::Point(
				x.try_into().expect("X coord should fit"),
				y.try_into().expect("Y coord should fit"),
			),
		})
	}
}

fn group_by_frequency(antennas: Vec<Option<Antenna>>) -> HashMap<String, Vec<Antenna>> {
	antennas
		.into_iter()
		.flatten()
		.into_group_map_by(|a| a.frequency.clone())
}

fn interference_points_at_distance_one(
	antennas: Vec<Antenna>,
	bounds: (Point, Point),
) -> impl Iterator<Item = Point> {
	antennas
		.into_iter()
		.combinations(2)
		.filter_map(move |pair| pair[0].interference_points_at_distance_one(&pair[1], bounds))
		.flatten()
}

fn interference_points(
	antennas: Vec<Antenna>,
	bounds: (Point, Point),
) -> impl Iterator<Item = Point> {
	antennas
		.into_iter()
		.combinations(2)
		.filter_map(move |pair| pair[0].interference_points(&pair[1], bounds))
		.flatten()
}

fn bounds(input: &str) -> (Point, Point) {
	let first_line = input
		.lines()
		.next()
		.expect("Should have at least one line!");
	let width: Num = first_line.len().try_into().expect("Coord should fit");
	let height: Num = input.lines().count().try_into().expect("Coord should fit");
	let max_bound = point::Point(width - 1, height - 1);
	(point::Point(0, 0), max_bound)
}

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	use crate::{
		day08_resonant_collinearity::{
			bounds, group_by_frequency, interference_points, interference_points_at_distance_one,
		},
		helpers::{self, types::point},
	};

	use super::try_parse;

	#[test]
	fn ex01() {
		let input = helpers::read::to_string("inputs/day08/ex01.txt");
		let bounds = bounds(&input);
		let antennas_by_frequency =
			group_by_frequency(helpers::parse_each_char::zero_indexed(&input, try_parse));
		let unique_interference_points = antennas_by_frequency
			.into_values()
			.flat_map(|antennas| interference_points_at_distance_one(antennas, bounds))
			.collect::<HashSet<_>>();
		let expected = HashSet::from_iter([
			point::Point(6, 0),
			point::Point(11, 0),
			point::Point(3, 1),
			point::Point(4, 2),
			point::Point(10, 2),
			point::Point(2, 3),
			point::Point(9, 4),
			point::Point(1, 5),
			point::Point(6, 5),
			point::Point(3, 6),
			point::Point(0, 7),
			point::Point(7, 7),
			point::Point(10, 10),
			point::Point(10, 11),
		]);
		assert_eq!(expected, unique_interference_points);
	}

	#[test]
	fn ex02() {
		let input = helpers::read::to_string("inputs/day08/ex02.txt");
		let bounds = bounds(&input);
		let antennas_by_frequency =
			group_by_frequency(helpers::parse_each_char::zero_indexed(&input, try_parse));
		let unique_interference_points = antennas_by_frequency
			.into_values()
			.flat_map(|antennas| interference_points_at_distance_one(antennas, bounds))
			.collect::<HashSet<_>>();
		let expected = HashSet::from_iter([point::Point(3, 1), point::Point(6, 7)]);
		assert_eq!(expected, unique_interference_points);
	}

	#[test]
	fn ex03() {
		let input = helpers::read::to_string("inputs/day08/ex03.txt");
		let bounds = bounds(&input);
		let antennas_by_frequency =
			group_by_frequency(helpers::parse_each_char::zero_indexed(&input, try_parse));
		let unique_interference_points = antennas_by_frequency
			.into_values()
			.flat_map(|antennas| interference_points_at_distance_one(antennas, bounds))
			.collect::<HashSet<_>>();
		let expected = HashSet::from_iter([
			point::Point(3, 1),
			point::Point(6, 7),
			point::Point(0, 2),
			point::Point(2, 6),
		]);
		assert_eq!(expected, unique_interference_points);
	}

	#[test]
	fn ex04() {
		let input = helpers::read::to_string("inputs/day08/ex04.txt");
		let bounds = bounds(&input);
		let antennas_by_frequency =
			group_by_frequency(helpers::parse_each_char::zero_indexed(&input, try_parse));
		let unique_interference_points = antennas_by_frequency
			.into_values()
			.flat_map(|antennas| interference_points_at_distance_one(antennas, bounds))
			.collect::<HashSet<_>>();
		let expected = HashSet::from_iter([
			point::Point(3, 1),
			point::Point(6, 7),
			point::Point(0, 2),
			point::Point(2, 6),
		]);
		assert_eq!(expected, unique_interference_points);
	}

	#[test]
	fn ex05() {
		let input = helpers::read::to_string("inputs/day08/ex05.txt");
		let bounds = bounds(&input);
		let antennas_by_frequency =
			group_by_frequency(helpers::parse_each_char::zero_indexed(&input, try_parse));
		let unique_interference_points = antennas_by_frequency
			.into_values()
			.flat_map(|antennas| interference_points(antennas, bounds))
			.collect::<HashSet<_>>();
		assert_eq!(9, unique_interference_points.len());
	}

	#[test]
	fn ex06() {
		let input = helpers::read::to_string("inputs/day08/ex01.txt");
		let bounds = bounds(&input);
		let antennas_by_frequency =
			group_by_frequency(helpers::parse_each_char::zero_indexed(&input, try_parse));
		let unique_interference_points = antennas_by_frequency
			.into_values()
			.flat_map(|antennas| interference_points(antennas, bounds))
			.collect::<HashSet<_>>();
		assert_eq!(34, unique_interference_points.len());
	}
}
