use core::str::FromStr;
use std::io::{BufRead, Lines};

use unicode_segmentation::UnicodeSegmentation as _;

/// A 2d grid.
///
/// Rows don't necessarily have to have the same number of columns.
#[derive(Debug, Clone)]
pub struct Grid<T> {
	data: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
	T: Clone,
{
	/// Get the value at the position, if it exists.
	pub fn get(&self, pos: &Pos) -> Option<&T> {
		let row_idx = usize::try_from(pos.y).ok()?;
		let col_idx = usize::try_from(pos.x).ok()?;
		self.data.get(row_idx).and_then(|row| row.get(col_idx))
	}

	/// Swaps the given value with the current value at the position, if it exists.
	///
	/// Returns the previous value at the position.
	/// No operation takes place if the previous position was not set.
	pub fn replace(&mut self, pos: &Pos, val: T) -> Option<T> {
		let row_idx = usize::try_from(pos.y).ok()?;
		let col_idx = usize::try_from(pos.x).ok()?;
		if let Some(current) = self
			.data
			.get_mut(row_idx)
			.and_then(|row| row.get_mut(col_idx))
		{
			let prev = current.clone();
			*current = val;
			Some(prev)
		} else {
			None
		}
	}

	/// Swap the values at two positions, if they exist.
	pub fn swap(&mut self, a: &Pos, b: &Pos) -> bool {
		if let Some(a_val) = self.get(a).cloned()
			&& let Some(b_val) = self.get(b).cloned()
		{
			self.replace(a, b_val);
			self.replace(b, a_val);
			true
		} else {
			false
		}
	}
}

impl<T> IntoIterator for &Grid<T> {
	type Item = Pos;

	type IntoIter = impl Iterator<Item = Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.data.iter().enumerate().flat_map(|(row_idx, row)| {
			row.iter().enumerate().map(move |(col_idx, _)| Pos {
				x: col_idx.try_into().expect("Wow x must be big"),
				y: row_idx.try_into().expect("Wow y must be big"),
			})
		})
	}
}

impl<B, T> TryFrom<Lines<B>> for Grid<T>
where
	B: BufRead,
	T: FromStr,
{
	type Error = T::Err;

	fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
		Ok(Grid {
			data: value
				.map(|line| {
					line
						.expect("Should have been able to read this line")
						.graphemes(true)
						.map(|s| T::from_str(s))
						.collect::<Result<Vec<_>, _>>()
				})
				.collect::<Result<Vec<_>, _>>()?,
		})
	}
}

/// An arbitrary 2d position that isn't bound checked to any grid.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Pos {
	x: isize,
	y: isize,
}

#[allow(non_snake_case, dead_code)]
pub fn Pos(x: isize, y: isize) -> Pos {
	Pos { x, y }
}

impl Pos {
	pub fn get_adjacent(&self, direction: Direction) -> Pos {
		match direction {
			Direction::Up => Pos {
				x: self.x,
				y: self.y - 1,
			},
			Direction::Down => Pos {
				x: self.x,
				y: self.y + 1,
			},
			Direction::Left => Pos {
				x: self.x - 1,
				y: self.y,
			},
			Direction::Right => Pos {
				x: self.x + 1,
				y: self.y,
			},
			Direction::UpLeft => Pos {
				x: self.x - 1,
				y: self.y - 1,
			},
			Direction::UpRight => Pos {
				x: self.x + 1,
				y: self.y - 1,
			},
			Direction::DownLeft => Pos {
				x: self.x - 1,
				y: self.y + 1,
			},
			Direction::DownRight => Pos {
				x: self.x + 1,
				y: self.y + 1,
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
	UpLeft,
	UpRight,
	DownLeft,
	DownRight,
}

impl Direction {
	pub fn clockwise() -> Vec<Self> {
		vec![
			Direction::Up,
			Direction::UpRight,
			Direction::Right,
			Direction::DownRight,
			Direction::Down,
			Direction::DownLeft,
			Direction::Left,
			Direction::UpLeft,
		]
	}

	pub fn turn_clockwise_90deg(self) -> Self {
		match self {
			Direction::Up => Direction::Right,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
			Direction::Right => Direction::Down,
			Direction::UpLeft => Direction::UpRight,
			Direction::UpRight => Direction::DownRight,
			Direction::DownLeft => Direction::UpLeft,
			Direction::DownRight => Direction::DownLeft,
		}
	}
}
