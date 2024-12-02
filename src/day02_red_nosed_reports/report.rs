use std::cmp::Ordering;

use super::trend::Trend;

#[derive(Debug)]
pub struct Report {
	levels: Vec<u32>,
	trend: Option<Trend>,
	min_step_size: Option<u32>,
	max_step_size: Option<u32>,
}

impl From<&[u32]> for Report {
	fn from(value: &[u32]) -> Self {
		let mut trend = None;
		let mut min_step_size = None;
		let mut max_step_size = None;
		for comp in value.windows(2) {
			let current_step = comp[0].abs_diff(comp[1]);
			if min_step_size.is_none_or(|prev| prev > current_step) {
				min_step_size = Some(current_step);
			}
			if max_step_size.is_none_or(|prev| prev < current_step) {
				max_step_size = Some(current_step);
			}
			trend = Some(match (trend, comp[0].cmp(&comp[1])) {
				(None, Ordering::Less) => Trend::StrictlyDecreasing,
				(None, Ordering::Equal) => Trend::Flat,
				(None, Ordering::Greater) => Trend::StrictlyIncreasing,
				(Some(Trend::StrictlyIncreasing), Ordering::Greater) => Trend::StrictlyIncreasing,
				(Some(Trend::StrictlyIncreasing), _) => Trend::Mixed,
				(Some(Trend::StrictlyDecreasing), Ordering::Less) => Trend::StrictlyDecreasing,
				(Some(Trend::StrictlyDecreasing), _) => Trend::Mixed,
				(Some(Trend::Flat), Ordering::Equal) => Trend::Flat,
				(Some(Trend::Flat), _) => Trend::Mixed,
				(Some(Trend::Mixed), _) => Trend::Mixed,
			})
		}

		Report {
			levels: value.to_owned(),
			trend,
			min_step_size,
			max_step_size,
		}
	}
}

impl Report {
	pub fn is_safe(&self) -> bool {
		(self.trend == Some(Trend::StrictlyIncreasing)
			|| self.trend == Some(Trend::StrictlyDecreasing))
			&& self.min_step_size >= Some(1)
			&& self.max_step_size <= Some(3)
	}

	pub fn is_safe_with_dampener(&self) -> bool {
		if self.is_safe() {
			return true;
		}

		(0..self.levels.len()).any(|skipped_idx| {
			Report::from(
				self
					.levels
					.iter()
					.enumerate()
					.filter_map(|(idx, elem)| {
						if idx == skipped_idx {
							None
						} else {
							Some(*elem)
						}
					})
					.collect::<Vec<_>>()
					.as_slice(),
			)
			.is_safe()
		})
	}
}