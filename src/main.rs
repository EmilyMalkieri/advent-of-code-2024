#![feature(let_chains)]

mod helpers;

mod day01_historian_hysteria;

fn main() {
	println!(
		"The similarity score: {}",
		day01_historian_hysteria::solve_2()
	)
}
