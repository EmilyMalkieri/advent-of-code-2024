use core::cmp::PartialOrd;
use core::ops::Mul;

use num::Num;

/// An abstract point in 2D space, unrelated to any grid.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point<N>
where
	N: Num + PartialOrd + Copy,
{
	x: N,
	y: N,
}

#[allow(non_snake_case)]
pub fn Point<N>(x: N, y: N) -> Point<N>
where
	N: Num + PartialOrd + Copy,
{
	Point { x, y }
}

impl<N> Point<N>
where
	N: Num + PartialOrd + Copy,
{
	pub fn is_bounded_by(&self, start: &Self, end: &Self) -> bool {
		self.x >= start.x && self.y >= start.y && self.x <= end.x && self.y <= end.y
	}

	pub fn distance_to(&self, other: &Self) -> Distance<N> {
		Distance {
			dx: other.x - self.x,
			dy: other.y - self.y,
		}
	}

	pub fn travel(&self, distance: &Distance<N>) -> Self {
		Self {
			x: self.x + distance.dx,
			y: self.y + distance.dy,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Distance<N>
where
	N: Num + PartialOrd + Copy,
{
	dx: N,
	dy: N,
}

impl<N> Mul<N> for Distance<N>
where
	N: Num + PartialOrd + Copy,
{
	type Output = Self;

	fn mul(self, rhs: N) -> Self::Output {
		Self {
			dx: self.dx * rhs,
			dy: self.dy * rhs,
		}
	}
}
