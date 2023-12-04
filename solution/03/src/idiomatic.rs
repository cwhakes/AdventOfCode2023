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

pub struct Schematic<'a>(Vec<&'a str>);

impl<'a> Schematic<'a> {
	fn new(s: &'a str) -> Self {
		Self(s.lines().collect())
	}

	fn get_number(&self, num: Number) -> i32 {
		let s = &self.0[num.start_y][num.start_x..][..num.len];
		s.parse().unwrap()
	}

	fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
		self.0.iter().enumerate().flat_map(|(y, line)| {
			line.chars().enumerate().filter_map(move |(x, c)| {
				(c != '.' && !c.is_ascii_digit()).then(move || Symbol { x, y, symbol: c })
			})
		})
	}

	fn numbers(&self) -> impl Iterator<Item = Number> + '_ {
		self.0.iter().enumerate().flat_map(|(y, line)| {
			let mut iter = line.chars().enumerate().fuse();
			let mut buf = 0;

			std::iter::from_fn(move || {
				while let Some((x, c)) = iter.next() {
					if c.is_ascii_digit() {
						buf += 1;
					} else if buf >= 1 {
						return Some(Number {
							start_x: x - buf,
							start_y: y,
							len: std::mem::take(&mut buf),
						});
					}
				}

				if buf >= 1 {
					Some(Number {
						start_x: line.len() - buf,
						start_y: y,
						len: std::mem::take(&mut buf),
					})
				} else {
					None
				}
			})
		})
	}
}

#[derive(Clone)]
struct Number {
	start_x: usize,
	start_y: usize,
	len: usize,
}

impl Number {
	fn is_adjacent_to(&self, (x, y): &(usize, usize)) -> bool {
		((self.start_x.saturating_sub(1))..=(self.start_x + self.len)).contains(x)
			&& ((self.start_y.saturating_sub(1))..=(self.start_y + 1)).contains(y)
	}
}

struct Symbol {
	x: usize,
	y: usize,
	symbol: char,
}

impl Symbol {
	fn is_gear(&self) -> bool {
		self.symbol == '*'
	}
}

fn get_answer1(input: &str) -> impl Display {
	let s = &Schematic::new(input);
	s.numbers()
		.filter(|n| s.symbols().any(|s| n.is_adjacent_to(&(s.x, s.y))))
		.map(|n| s.get_number(n))
		.sum::<i32>()
}

fn get_answer2(input: &str) -> impl Display {
	let s = &Schematic::new(input);
	s.symbols()
		.filter(Symbol::is_gear)
		.filter_map(|g| {
			let nums: Vec<Number> = s
				.numbers()
				.filter(|n| n.is_adjacent_to(&(g.x, g.y)))
				.collect();
			(nums.len() == 2).then(|| nums.into_iter().map(|n| s.get_number(n)).product::<i32>())
		})
		.sum::<i32>()
}
