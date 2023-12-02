use std::fmt::Display;
use std::fs::File;
use std::io::Read;

fn main() {
	let mut buf = String::new();
	let mut file = File::open("input/01/input").unwrap();
	file.read_to_string(&mut buf).unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let mut num = 0;
	for line in input.lines() {
		let digits = line.chars().filter(char::is_ascii_digit);
		let first = digits.clone().next().unwrap().to_digit(10).unwrap();
		let last = digits.clone().next_back().unwrap().to_digit(10).unwrap();
		num += first * 10 + last;
	}
	num
}

fn get_answer2(input: &str) -> impl Display {
	let array = [
		"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
	]
	.into_iter()
	.map(|n| n.chars().collect::<Vec<_>>())
	.collect::<Vec<_>>();

	let mut num = 0;
	for line in input.lines() {
		let mut line: Vec<char> = line.chars().collect();

		'outer: for i in 0..(line.len()) {
			for (n, txt) in array.iter().enumerate() {
				if line[i..].starts_with(txt) {
					line[i] = char::from_digit(n as u32 + 1, 10).unwrap();
					break 'outer;
				}
			}
		}
		'outer: for i in (0..(line.len())).rev() {
			for (n, txt) in array.iter().enumerate() {
				if line[..=i].ends_with(txt) {
					line[i] = char::from_digit(n as u32 + 1, 10).unwrap();
					break 'outer;
				}
			}
		}

		let digits = line.into_iter().filter(char::is_ascii_digit);
		let first = digits.clone().next().unwrap().to_digit(10).unwrap();
		let last = digits.clone().next_back().unwrap().to_digit(10).unwrap();
		num += first * 10 + last;
	}
	num
}
