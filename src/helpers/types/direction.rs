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
	pub fn reverse(self) -> Self {
		match self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
			Direction::UpLeft => Direction::DownRight,
			Direction::UpRight => Direction::DownLeft,
			Direction::DownLeft => Direction::UpRight,
			Direction::DownRight => Direction::UpLeft,
		}
	}

	pub fn clockwise_90deg() -> [Self; 4] {
		[
			Direction::Up,
			Direction::Right,
			Direction::Down,
			Direction::Left,
		]
	}

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
