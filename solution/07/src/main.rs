#[cfg(test)]
mod test;

use std::fmt::Display;
use std::fs;

fn main() {
	let buf = fs::read_to_string("input/07/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Card(u8);

impl From<char> for Card {
	fn from(c: char) -> Self {
		match c {
			'2' => Self(2),
			'3' => Self(3),
			'4' => Self(4),
			'5' => Self(5),
			'6' => Self(6),
			'7' => Self(7),
			'8' => Self(8),
			'9' => Self(9),
			'T' => Self(10),
			'J' => Self(11),
			'Q' => Self(12),
			'K' => Self(13),
			'A' => Self(14),
			_ => panic!("{}", c),
		}
	}
}

impl Card {
	fn jokerify(mut self) -> Self {
		if self.0 == 11 {
			self.0 = 1;
		}
		self
	}

	fn is_joker(&self) -> bool {
		self.0 == 1
	}
}

#[derive(Debug, PartialEq, Eq)]
struct Hand([Card; 5]);

impl From<&str> for Hand {
	fn from(value: &str) -> Self {
		let mut iter = value.chars().map(Card::from);
		Self([
			iter.next().unwrap(),
			iter.next().unwrap(),
			iter.next().unwrap(),
			iter.next().unwrap(),
			iter.next().unwrap(),
		])
	}
}

impl Hand {
	fn jokerify(self) -> Self {
		Self(self.0.map(Card::jokerify))
	}

	fn get_type(&self) -> Type {
		let mut inner = self.0.clone();
		inner.sort();
		let mut counts = [0; 5];

		let mut i = 0;
		let mut needle = &inner[0];
		for card in &inner {
			if !card.is_joker() {
				if card == needle {
					counts[i] += 1;
				} else {
					i += 1;
					counts[i] += 1;
					needle = card;
				}
			}
		}
		counts.sort();
		let jokers = inner.into_iter().filter(Card::is_joker).count();
		counts[4] += jokers;

		match counts {
			[0, 0, 0, 0, 5] => Type::FiveOfAKind,
			[0, 0, 0, 1, 4] => Type::FourOfAKind,
			[0, 0, 0, 2, 3] => Type::FullHouse,
			[0, 0, 1, 1, 3] => Type::ThreeOfAKind,
			[0, 0, 1, 2, 2] => Type::TwoPair,
			[0, 1, 1, 1, 2] => Type::OnePair,
			[1, 1, 1, 1, 1] => Type::HighCard,
			_ => unreachable!(),
		}
	}
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match self.get_type().cmp(&other.get_type()) {
			std::cmp::Ordering::Less => std::cmp::Ordering::Less,
			std::cmp::Ordering::Equal => self.0.cmp(&other.0),
			std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
		}
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

fn get_answer1(input: &str) -> impl Display {
	let mut hands = input
		.lines()
		.map(|s| {
			let (hand, bet) = s.split_once(' ').unwrap();
			let hand = Hand::from(hand);
			let bet = bet.parse::<usize>().unwrap();
			(hand, bet)
		})
		.collect::<Vec<_>>();

	hands.sort();

	hands
		.into_iter()
		.enumerate()
		.map(|(n, (_, bet))| (n + 1) * bet)
		.sum::<usize>()
}

fn get_answer2(input: &str) -> impl Display {
	let mut hands = input
		.lines()
		.map(|s| {
			let (hand, bet) = s.split_once(' ').unwrap();
			let hand = Hand::from(hand).jokerify();
			let bet = bet.parse::<usize>().unwrap();
			(hand, bet)
		})
		.collect::<Vec<_>>();

	hands.sort();
	hands
		.into_iter()
		.enumerate()
		.map(|(n, (_, bet))| (n + 1) * bet)
		.sum::<usize>()
}
