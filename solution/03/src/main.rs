#[cfg(test)]
mod test;

use std::fmt::Display;
use std::fs;

fn main() {
	let buf = fs::read_to_string("input/03/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

struct Number {
	number: i32,
	start_x: isize,
	start_y: isize,
	len: usize,
}

impl Number {
	fn find(s: &str) -> Vec<Self> {
		let mut vec = Vec::new();
		let mut buf = String::new();
		for (y, line) in s.lines().enumerate() {
			for (x, c) in line.chars().enumerate() {
				if c.is_digit(10) {
					buf.push(c)
				} else {
					if !buf.is_empty() {
						let number = buf.parse::<i32>().unwrap();
						let start_x = (x - buf.len()) as isize;
						let start_y = y as isize;
						let len = buf.len();
						vec.push(Self {
							number,
							start_x,
							start_y,
							len,
						});
						buf.clear();
					}
				}
			}

			if !buf.is_empty() {
				let number = buf.parse::<i32>().unwrap();
				let start_x = (line.len() - buf.len()) as isize;
				let start_y = y as isize;
				let len = buf.len();
				vec.push(Self {
					number,
					start_x,
					start_y,
					len,
				});
				buf.clear();
			}
		}

		vec
	}

	fn is_adjacent_to(&self, (x, y): &(isize, isize)) -> bool {
		((self.start_x - 1)..=(self.start_x + self.len as isize)).contains(x)
			&& ((self.start_y - 1)..=(self.start_y + 1)).contains(y)
	}
}

fn find_symbols(s: &str) -> Vec<(isize, isize)> {
	let mut vec = Vec::new();
	for (y, line) in s.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c != '.' && !c.is_digit(10) {
				vec.push((x as isize, y as isize));
			}
		}
	}
	vec
}

fn find_possible_gears(s: &str) -> Vec<(isize, isize)> {
	let mut vec = Vec::new();
	for (y, line) in s.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c == '*' {
				vec.push((x as isize, y as isize));
			}
		}
	}
	vec
}

fn get_answer1(input: &str) -> impl Display {
	let numbers = Number::find(input);
	let symbols = find_symbols(input);
	numbers
		.into_iter()
		.filter(|n| symbols.iter().any(|s| n.is_adjacent_to(s)))
		.map(|n| n.number)
		.sum::<i32>()
}

fn get_answer2(input: &str) -> impl Display {
	let numbers = Number::find(input);
	let possible_gears = find_possible_gears(input);
	possible_gears
		.into_iter()
		.filter_map(|s| {
			let nums = numbers.iter().filter(|n| n.is_adjacent_to(&s));
			(nums.clone().count() == 2).then(|| nums.map(|n| n.number).product::<i32>())
		})
		.sum::<i32>()
}
