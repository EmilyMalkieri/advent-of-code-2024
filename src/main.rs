#![feature(let_chains)]

mod helpers;

mod day01_historian_hysteria;
mod day02_red_nosed_reports;
mod day03_mull_it_over;

fn main() {
	println!(
		"All the multiplications add up to {}.",
		day03_mull_it_over::solve_1()
	)
}
