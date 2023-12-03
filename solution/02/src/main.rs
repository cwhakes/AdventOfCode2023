#[cfg(test)]
mod test;

use std::fmt::Display;
use std::{cmp, fs};

fn main() {
	let buf = fs::read_to_string("input/02/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

#[derive(Default, Debug)]
struct Game {
	red: u32,
	green: u32,
	blue: u32,
}

impl Game {
	fn new(s: &str) -> (usize, Vec<Self>) {
		let s = s.strip_prefix("Game ").unwrap();
		let (n, s) = s.split_once(": ").unwrap();
		let n = n.parse().ok().unwrap();

		let vec = s
			.split("; ")
			.map(|grab| {
				let mut game = Game::default();
				for cube in grab.split(", ") {
					match cube.split_once(" ").unwrap() {
						(n, "red") => game.red += n.parse::<u32>().unwrap(),
						(n, "green") => game.green += n.parse::<u32>().unwrap(),
						(n, "blue") => game.blue += n.parse::<u32>().unwrap(),
						_ => panic!(),
					}
				}
				game
			})
			.collect();

		(n, vec)
	}

	fn is_subset_of(&self, parent: &Self) -> bool {
		parent.red >= self.red && parent.green >= self.green && parent.blue >= self.blue
	}

	fn max(games: impl IntoIterator<Item = Game>) -> Game {
		let mut max_game = Game::default();
		for game in games {
			max_game.red = cmp::max(max_game.red, game.red);
			max_game.green = cmp::max(max_game.green, game.green);
			max_game.blue = cmp::max(max_game.blue, game.blue);
		}
		max_game
	}

	fn power(&self) -> u32 {
		self.red * self.blue * self.green
	}
}

fn get_answer1(input: &str) -> impl Display {
	let parent = Game {
		red: 12,
		green: 13,
		blue: 14,
	};

	input
		.lines()
		.map(Game::new)
		.filter(|(_, games)| games.iter().all(|g| g.is_subset_of(&parent)))
		.map(|(n, _)| n)
		.sum::<usize>()
}

fn get_answer2(input: &str) -> impl Display {
	input
		.lines()
		.map(Game::new)
		.map(|(_, games)| Game::max(games).power())
		.sum::<u32>()
}
