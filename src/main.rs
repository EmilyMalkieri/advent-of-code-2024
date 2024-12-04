#![warn(
	clippy::pedantic,
	clippy::unused_trait_names,
	clippy::pattern_type_mismatch,
	clippy::absolute_paths,
	clippy::std_instead_of_core,
	clippy::unwrap_used,
	clippy::todo
)]
#![feature(let_chains)]
#![feature(impl_trait_in_assoc_type)]

mod helpers;

mod day01_historian_hysteria;
mod day02_red_nosed_reports;
mod day03_mull_it_over;
mod day04_ceres_search;

fn main() {
	println!(
		"XMAS appears {} times in our word search.",
		day04_ceres_search::solve_1()
	);
}
