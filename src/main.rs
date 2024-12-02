#![feature(let_chains)]

mod helpers;

mod day01_historian_hysteria;
mod day02_red_nosed_reports;

fn main() {
	println!(
		"{} of the reports are safe.",
		day02_red_nosed_reports::solve_1()
	)
}
