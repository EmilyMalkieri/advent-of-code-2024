use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Trend {
	StrictlyIncreasing,
	StrictlyDecreasing,
	Mixed,
	Flat,
}

impl From<Ordering> for Trend {
	fn from(value: Ordering) -> Self {
		match value {
			Ordering::Less => Trend::StrictlyDecreasing,
			Ordering::Equal => Trend::Flat,
			Ordering::Greater => Trend::StrictlyIncreasing,
		}
	}
}

impl Trend {
	pub fn adjust(&self, direction: Ordering) -> Trend {
		match (self, direction) {
			(Trend::StrictlyIncreasing, Ordering::Greater) => Trend::StrictlyIncreasing,
			(Trend::StrictlyIncreasing, _) => Trend::Mixed,
			(Trend::StrictlyDecreasing, Ordering::Less) => Trend::StrictlyDecreasing,
			(Trend::StrictlyDecreasing, _) => Trend::Mixed,
			(Trend::Flat, Ordering::Equal) => Trend::Flat,
			(Trend::Flat, _) => Trend::Mixed,
			(Trend::Mixed, _) => Trend::Mixed,
		}
	}
}
