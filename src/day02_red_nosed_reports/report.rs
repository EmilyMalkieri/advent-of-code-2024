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
		let mut trend: Option<Trend> = None;
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
			let direction = comp[0].cmp(&comp[1]);
			trend = trend.map_or_else(
				|| Some(Trend::from(direction)),
				|current| Some(current.adjust(direction)),
			);
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
			let mut levels = self.levels.clone();
			levels.remove(skipped_idx);
			Report::from(levels.as_slice()).is_safe()
		})
	}
}
