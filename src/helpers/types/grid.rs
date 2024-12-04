use std::{
	borrow::ToOwned,
	io::{BufRead, Lines},
};

use unicode_segmentation::UnicodeSegmentation as _;

/// A 2d grid.
///
/// `T` shouldn't be an Option or you won't be able to tell the difference between `get` returning None because the position is invalid or `get` returning None because you've set that field to None.
///
/// Rows don't necessarily have to have the same number of columns.
#[derive(Debug)]
pub struct Grid<T> {
	data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
	/// Get the value at the position, if it exists.
	pub fn get(&self, pos: &Pos) -> Option<&T> {
		let row_idx = usize::try_from(pos.y).ok()?;
		let col_idx = usize::try_from(pos.x).ok()?;
		self.data.get(row_idx).and_then(|row| row.get(col_idx))
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

impl<B> From<Lines<B>> for Grid<String>
where
	B: BufRead,
{
	fn from(value: Lines<B>) -> Self {
		Grid {
			data: value
				.map(|line| {
					line
						.expect("Unable to read line")
						.graphemes(true)
						.map(ToOwned::to_owned)
						.collect()
				})
				.collect(),
		}
	}
}

/// An arbitrary 2d position that isn't bound checked to any grid.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pos {
	x: isize,
	y: isize,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
}
