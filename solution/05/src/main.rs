#[cfg(test)]
mod test;

use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
use std::ops::Range;

fn main() {
	let buf = fs::read_to_string("input/05/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

struct Map {
	parts: Vec<MapPart>,
}

impl Map {
	fn new(s: &str) -> Self {
		let mut iter = s.lines();
		let _header = iter.next().unwrap();
		let parts = iter.map(MapPart::new).collect();
		Self { parts }
	}

	fn convert(&self, n: usize) -> usize {
		self.parts
			.iter()
			.find_map(|part| part.convert(n))
			.unwrap_or(n)
	}

	fn convert_range(&self, range: Range<usize>) -> HashSet<Range<usize>> {
		let mut inputs = vec![range];
		let mut outputs = HashSet::new();
		for part in &self.parts {
			let mut leftovers = Vec::new();
			for input in inputs.drain(..) {
				let ((head, tail), body) = part.convert_range(input);
				if !head.is_empty() {
					leftovers.push(head);
				}
				if !tail.is_empty() {
					leftovers.push(tail);
				}
				if !body.is_empty() {
					outputs.insert(body);
				}
			}
			inputs = leftovers;
		}
		outputs.extend(inputs);
		outputs
	}
}

struct MapPart {
	output_start: usize,
	input_start: usize,
	len: usize,
}

impl MapPart {
	fn new(s: &str) -> Self {
		let mut iter = s.split_whitespace();
		let output_start = iter.next().unwrap().parse().unwrap();
		let input_start = iter.next().unwrap().parse().unwrap();
		let len = iter.next().unwrap().parse().unwrap();
		assert!(iter.next().is_none());
		Self {
			output_start,
			input_start,
			len,
		}
	}

	fn input_end(&self) -> usize {
		self.input_start + self.len
	}

	fn convert(&self, n: usize) -> Option<usize> {
		(self.input_start..self.input_end())
			.contains(&n)
			.then_some(n - self.input_start + self.output_start)
	}

	fn convert_range(&self, range: Range<usize>) -> ((Range<usize>, Range<usize>), Range<usize>) {
		let head = range.start..std::cmp::min(self.input_start, range.end);
		let tail = std::cmp::max(range.start, self.input_end())..range.end;

		let body_start = if let Some(converted_start) = self.convert(range.start) {
			std::cmp::max(converted_start, self.output_start)
		} else {
			self.output_start
		};

		let len = std::cmp::min(self.input_end(), range.end)
			.saturating_sub(std::cmp::max(range.start, self.input_start));
		let body = body_start..(body_start + len);

		((head, tail), body)
	}
}

fn get_answer1(input: &str) -> impl Display {
	let mut iter = input.split("\n\n");
	let seeds = iter.next().unwrap();
	let seeds = seeds
		.strip_prefix("seeds: ")
		.unwrap()
		.split_whitespace()
		.map(|s| s.parse::<usize>().unwrap())
		.collect::<Vec<_>>();

	let maps = iter.map(Map::new).collect::<Vec<_>>();

	let mut destinations = seeds.clone();

	for map in maps {
		dbg!(&destinations);
		destinations = destinations.into_iter().map(|d| map.convert(d)).collect()
	}

	dbg!(&destinations);
	destinations.into_iter().min().unwrap()
}

fn get_answer2(input: &str) -> impl Display {
	let mut iter = input.split("\n\n");
	let seeds = iter.next().unwrap();
	let seeds = seeds
		.strip_prefix("seeds: ")
		.unwrap()
		.split_whitespace()
		.map(|s| s.parse::<usize>().unwrap())
		.collect::<Vec<_>>();

	let seed_ranges = seeds
		.chunks(2)
		.map(|s| s[0]..(s[0] + s[1]))
		.collect::<HashSet<_>>();

	let maps = iter.map(Map::new).collect::<Vec<_>>();

	let mut destinations = seed_ranges.clone();

	for map in maps {
		dbg!(&destinations);

		destinations = destinations
			.into_iter()
			.flat_map(|d| map.convert_range(d))
			.collect()
	}

	dbg!(&destinations);
	destinations.into_iter().map(|r| r.start).min().unwrap()
}
