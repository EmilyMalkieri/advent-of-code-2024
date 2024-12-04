#![warn(
	clippy::pedantic,
	clippy::unused_trait_names,
	clippy::pattern_type_mismatch,
	clippy::absolute_paths,
	clippy::std_instead_of_core,
	clippy::unwrap_used
)]
#![feature(let_chains)]

mod helpers;

mod day01_historian_hysteria;
mod day02_red_nosed_reports;
mod day03_mull_it_over;

fn main() {
	println!(
		"All the enabled multiplications add up to {}.",
		day03_mull_it_over::solve_2()
	);
}
