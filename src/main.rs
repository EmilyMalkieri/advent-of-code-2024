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
mod day10_hoof_it;
mod day11_plutonian_pebbles;
mod day12_garden_groups;

fn main() {
	println!(
		"We need to spend {} for all fences (bulk discount!!!).",
		day12_garden_groups::solve_2()
	);
}
