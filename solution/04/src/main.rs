#[cfg(test)]
mod test;

use std::collections::HashSet;
use std::fmt::Display;
use std::fs;

fn main() {
	let buf = fs::read_to_string("input/04/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

fn parse(s: &str) -> (i32, HashSet<i32>, HashSet<i32>) {
	let s = s.strip_prefix("Card").unwrap();
	let (num, s) = s.split_once(":").unwrap();
	let num = num.trim().parse::<i32>().unwrap();
	let (winners, numbers) = s.split_once("|").unwrap();
	let winners: HashSet<i32> = winners
		.trim()
		.split_whitespace()
		.map(|s| s.parse().unwrap())
		.collect();
	let numbers: HashSet<i32> = numbers
		.trim()
		.split_whitespace()
		.map(|s| s.parse().unwrap())
		.collect();
	(num, winners, numbers)
}

fn get_answer1(input: &str) -> impl Display {
	input
		.lines()
		.map(parse)
		.map(|(_, w, n)| 2i32.pow(w.intersection(&n).count() as u32) / 2)
		.sum::<i32>()
}

fn get_answer2(input: &str) -> impl Display {
	let cards = input.lines().map(parse).collect::<Vec<_>>();

	let mut wins = vec![1; cards.len()];

	for i in 0..cards.len() {
		let count = cards[i].1.intersection(&cards[i].2).count();

		for j in (i + 1)..(i + count + 1) {
			wins[j] += wins[i]
		}
	}

	wins.into_iter().sum::<i64>()
}
