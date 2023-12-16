#[cfg(test)]
mod test;

use std::fmt::Display;
use std::fs;

fn main() {
	let buf = fs::read_to_string("input/11/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

struct Pos {
	x: i128,
	y: i128,
}

impl Pos {
	fn new(x: usize, y: usize) -> Self {
		Self {
			x: x as i128,
			y: y as i128,
		}
	}
}

struct Space {
	rows: Vec<i128>,
	cols: Vec<i128>,
}

impl Space {
	fn distance_between(&self, a: &Pos, b: &Pos) -> i128 {
		let base = (b.x - a.x).abs() + (b.y - a.y).abs();
		let expanded_x = self
			.cols
			.iter()
			.filter(|x| (a.x..=b.x).contains(x) | (b.x..=a.x).contains(x))
			.count();
		let expanded_y = self
			.rows
			.iter()
			.filter(|y| (a.y..=b.y).contains(y) | (b.y..=a.y).contains(y))
			.count();

		base + expanded_x as i128 + expanded_y as i128
	}

	fn distance_between_big(&self, a: &Pos, b: &Pos) -> i128 {
		let base = (b.x - a.x).abs() + (b.y - a.y).abs();
		let expanded_x = self
			.cols
			.iter()
			.filter(|x| (a.x..=b.x).contains(x) | (b.x..=a.x).contains(x))
			.count();
		let expanded_y = self
			.rows
			.iter()
			.filter(|y| (a.y..=b.y).contains(y) | (b.y..=a.y).contains(y))
			.count();

		base + (expanded_x as i128 + expanded_y as i128) * (1000000 - 1)
	}
}

fn galaxies(s: &str) -> (Vec<Pos>, Space) {
	let galaxies = s
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.chars()
				.enumerate()
				.filter(|(_, c)| *c == '#')
				.map(move |(x, _)| Pos::new(x, y))
		})
		.collect::<Vec<_>>();

	let max_x = galaxies.iter().map(|pos| pos.x).max().unwrap();
	let max_y = galaxies.iter().map(|pos| pos.y).max().unwrap();

	let cols = (0..max_x)
		.filter(|x| galaxies.iter().map(|pos| pos.x).all(|px| *x != px))
		.collect::<Vec<_>>();

	let rows = (0..max_y)
		.filter(|y| galaxies.iter().map(|pos| pos.y).all(|py| *y != py))
		.collect::<Vec<_>>();

	(galaxies, Space { rows, cols })
}

fn get_answer1(input: &str) -> impl Display {
	let (galaxies, space) = galaxies(input);

	let mut sum = 0;
	for i in 0..galaxies.len() {
		for j in (i + 1)..galaxies.len() {
			sum += space.distance_between(&galaxies[i], &galaxies[j]);
		}
	}
	sum
}

fn get_answer2(input: &str) -> impl Display {
	let (galaxies, space) = galaxies(input);

	let mut sum = 0;
	for i in 0..galaxies.len() {
		for j in (i + 1)..galaxies.len() {
			sum += space.distance_between_big(&galaxies[i], &galaxies[j]);
		}
	}
	sum
}
