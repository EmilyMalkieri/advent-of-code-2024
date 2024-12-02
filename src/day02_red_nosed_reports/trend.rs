#[derive(Debug, PartialEq, Eq)]
pub enum Trend {
	StrictlyIncreasing,
	StrictlyDecreasing,
	Mixed,
	Flat,
}
