#[cfg(test)]
mod test;

use std::fmt::Display;
use std::fs;

fn main() {
	let buf = fs::read_to_string("input/09/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

struct Sequence(Vec<i64>);

impl Sequence {
	fn new(s: &str) -> Self {
		Self(s.split_whitespace().map(|n| n.parse().unwrap()).collect())
	}

	fn diff(&self) -> Self {
		Self(
			(1..self.0.len())
				.map(|i| self.0[i] - self.0[i - 1])
				.collect(),
		)
	}

	fn predict(&self) -> i64 {
		if self.0.iter().all(|&n| n == 0) {
			0
		} else {
			self.0[self.0.len() - 1] + self.diff().predict()
		}
	}

	fn predict_backwards(&self) -> i64 {
		if self.0.iter().all(|&n| n == 0) {
			0
		} else {
			self.0[0] - self.diff().predict_backwards()
		}
	}
}

fn get_answer1(input: &str) -> impl Display {
	input
		.lines()
		.map(Sequence::new)
		.map(|s| s.predict())
		.sum::<i64>()
}

fn get_answer2(input: &str) -> impl Display {
	input
		.lines()
		.map(Sequence::new)
		.map(|s| s.predict_backwards())
		.sum::<i64>()
}
