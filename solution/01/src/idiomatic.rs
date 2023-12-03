#[cfg(test)]
mod test;

use std::fmt::Display;
use std::fs;

fn main() {
	let buf = fs::read_to_string("input/01/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

fn get_value(s: &str, map: &[(&str, u32)]) -> Option<u32> {
	let first = map
		.iter()
		.filter_map(|(pat, n)| Some((s.find(pat)?, *n)))
		.min();
	let last = map
		.iter()
		.filter_map(|(pat, n)| Some((s.rfind(pat)?, *n)))
		.max();
	Some(first?.1 * 10 + last?.1)
}

fn get_answer1(input: &str) -> impl Display {
	let map = [
		("0", 0),
		("1", 1),
		("2", 2),
		("3", 3),
		("4", 4),
		("5", 5),
		("6", 6),
		("7", 7),
		("8", 8),
		("9", 9),
	];

	input
		.lines()
		.map(|line| get_value(line, &map).unwrap())
		.sum::<u32>()
}

fn get_answer2(input: &str) -> impl Display {
	let map = [
		("0", 0),
		("1", 1),
		("2", 2),
		("3", 3),
		("4", 4),
		("5", 5),
		("6", 6),
		("7", 7),
		("8", 8),
		("9", 9),
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
	];

	input
		.lines()
		.map(|line| get_value(line, &map).unwrap())
		.sum::<u32>()
}
