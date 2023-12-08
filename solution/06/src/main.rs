#[cfg(test)]
mod test;

use std::fmt::Display;
use std::fs;

fn main() {
	let buf = fs::read_to_string("input/06/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

struct Race {
	time: i64,
	distance: i64,
}

impl Race {
	fn new(s: &str) -> Self {
		let mut iter = s.lines();
		let time = iter
			.next()
			.unwrap()
			.split_whitespace()
			.skip(1)
			.collect::<String>()
			.parse::<i64>()
			.unwrap();
		let distance = iter
			.next()
			.unwrap()
			.split_whitespace()
			.skip(1)
			.collect::<String>()
			.parse::<i64>()
			.unwrap();
		Self { time, distance }
	}

	fn new_many(s: &str) -> Vec<Self> {
		let mut iter = s.lines();
		let times = iter
			.next()
			.unwrap()
			.split_whitespace()
			.skip(1)
			.map(|s| s.parse::<i64>().unwrap());
		let distances = iter
			.next()
			.unwrap()
			.split_whitespace()
			.skip(1)
			.map(|s| s.parse::<i64>().unwrap());
		times
			.zip(distances)
			.map(|(t, d)| Race {
				time: t,
				distance: d,
			})
			.collect()
	}

	fn count(&self) -> usize {
		let t = self.time as f64;
		let d = self.distance as f64;

		let root1 = (t - (t.powi(2) - 4.0 * d).sqrt()) / 2.0;
		let root2 = (t + (t.powi(2) - 4.0 * d).sqrt()) / 2.0;

		((root2 - 1.0).ceil() - (root1 + 1.0).floor() + 1.0) as usize
	}
}

fn get_answer1(input: &str) -> impl Display {
	Race::new_many(input)
		.iter()
		.map(Race::count)
		.product::<usize>()
}

fn get_answer2(input: &str) -> impl Display {
	Race::new(input).count()
}
