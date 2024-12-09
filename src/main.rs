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
mod day05_print_queue;
mod day06_guard_gallivant;
mod day07_bridge_repair;
mod day08_resonant_collinearity;
mod day09_disk_fragmenter;

fn main() {
	println!(
		"After moving but without fragmenting, the filesystem's checksum is {}",
		day09_disk_fragmenter::solve_2()
	);
}
