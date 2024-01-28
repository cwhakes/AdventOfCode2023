#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

fn main() {
	let buf = fs::read_to_string("input/12/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Spring {
	Operational,
	Damaged,
	Unknown,
}

impl From<char> for Spring {
	fn from(value: char) -> Self {
		match value {
			'.' => Self::Operational,
			'#' => Self::Damaged,
			'?' => Self::Unknown,
			_ => panic!("Unknown character: {value}"),
		}
	}
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Springs<'a>(&'a [Spring]);

impl<'a> From<&'a [Spring]> for Springs<'a> {
	fn from(value: &'a [Spring]) -> Self {
		Self(value)
	}
}

impl<'a> Springs<'a> {
	fn normalize(&self) -> Self {
		let n = self
			.0
			.iter()
			.take_while(|&s| s == &Spring::Operational)
			.count();
		Self(&self.0[n..])
	}

	fn find_subgroups(&self, len: usize) -> impl '_ + Iterator<Item = Springs<'a>> {
		(0..=(self.0.len().saturating_sub(len)))
			.map(|n| self.0.split_at(n))
			.take_while(move |(head, body)| {
				body.len() >= len && head.iter().all(|s| s != &Spring::Damaged)
			})
			.map(move |(_head, body)| body.split_at(len))
			.filter(|(body, _tail)| body.iter().all(|s| s != &Spring::Operational))
			.filter_map(|(_body, tail)| {
				if let Some((seperator, tail)) = tail.split_first() {
					(seperator != &Spring::Damaged).then_some(Springs::from(tail))
				} else {
					Some(Springs::default())
				}
			})
	}

	fn count_arrangements(
		&self,
		groups: &'a [usize],
		memos: &mut HashMap<(Self, &'a [usize]), usize>,
	) -> usize {
		let springs = self.normalize();

		if let Some((&head, tail)) = groups.split_first() {
			if let Some(&memo) = memos.get(&(springs.clone(), groups)) {
				return memo;
			}

			let count = springs
				.find_subgroups(head)
				.map(|sub_springs| sub_springs.count_arrangements(tail, memos))
				.sum::<usize>();

			memos.insert((springs, groups), count);

			count
		} else {
			usize::from(self.0.iter().all(|s| s != &Spring::Damaged))
		}
	}
}

fn parse_line(line: &str) -> (Vec<Spring>, Vec<usize>) {
	let (springs, groups) = line.split_once(' ').unwrap();
	let springs = springs.chars().map(Spring::from).collect();
	let groups = groups
		.split(',')
		.map(|n| n.parse::<usize>().unwrap())
		.collect();
	(springs, groups)
}

fn parse_line2(line: &str) -> (Vec<Spring>, Vec<usize>) {
	let (springs, groups) = line.split_once(' ').unwrap();
	let springs = format!("{0}?{0}?{0}?{0}?{0}", springs);
	let springs = springs.chars().map(Spring::from).collect();
	let groups = format!("{0},{0},{0},{0},{0}", groups);
	let groups = groups
		.split(',')
		.map(|n| n.parse::<usize>().unwrap())
		.collect();
	(springs, groups)
}

fn get_answer1(input: &str) -> impl Display {
	input
		.lines()
		.map(parse_line)
		.map(|(s, g)| {
			let mut memos = HashMap::new();
			Springs::from(&*s).count_arrangements(&g, &mut memos)
		})
		.sum::<usize>()
}

fn get_answer2(input: &str) -> impl Display {
	input
		.lines()
		.map(parse_line2)
		.map(|(s, g)| {
			let mut memos = HashMap::new();
			Springs::from(&*s).count_arrangements(&g, &mut memos)
		})
		.sum::<usize>()
}
