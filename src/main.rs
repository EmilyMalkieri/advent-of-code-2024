#![feature(let_chains)]

mod helpers;

mod day01_historian_hysteria;
mod day02_red_nosed_reports;
mod day03_mull_it_over;

fn main() {
	println!(
		"{} of the reports are safe (with dampener on).",
		day02_red_nosed_reports::solve_2()
	)
}
