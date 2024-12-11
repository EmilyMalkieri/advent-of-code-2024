use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Height {
	Height(u8),
	Void,
}

impl FromStr for Height {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"0" => Ok(Height::Height(0)),
			"1" => Ok(Height::Height(1)),
			"2" => Ok(Height::Height(2)),
			"3" => Ok(Height::Height(3)),
			"4" => Ok(Height::Height(4)),
			"5" => Ok(Height::Height(5)),
			"6" => Ok(Height::Height(6)),
			"7" => Ok(Height::Height(7)),
			"8" => Ok(Height::Height(8)),
			"9" => Ok(Height::Height(9)),
			"." => Ok(Height::Void),
			other => Err(format!("Unexpected value {other}")),
		}
	}
}
