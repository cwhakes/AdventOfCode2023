#[cfg(test)]
mod test;

use std::collections::BinaryHeap;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

fn main() {
	let buf = fs::read_to_string("input/07/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Card(u8);

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

impl TryFrom<char> for Card {
	type Error = ();

	fn try_from(c: char) -> Result<Self, Self::Error> {
		Ok(Self(match c {
			'2' => 2,
			'3' => 3,
			'4' => 4,
			'5' => 5,
			'6' => 6,
			'7' => 7,
			'8' => 8,
			'9' => 9,
			'T' => 10,
			'J' => 11,
			'Q' => 12,
			'K' => 13,
			'A' => 14,
			_ => return Err(()),
		}))
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

#[derive(Debug, PartialEq, Eq)]
struct Hand([Card; 5]);

impl Hand {
	fn jokerify(self) -> Self {
		Self(self.0.map(Card::jokerify))
	}

	fn count_jokers(&self) -> usize {
		self.0.iter().filter(|c| c.is_joker()).count()
	}

	fn get_type(&self) -> Type {
		let mut inner = self.0.clone();
		inner.sort();

		let mut counts = [0; 5];
		let mut i = 0;
		let mut needle = &inner[0];
		for card in inner.iter().filter(|c| !c.is_joker()) {
			if needle != card {
				i += 1;
				needle = card;
			}
			counts[i] += 1;
		}
		counts.sort();
		counts[4] += self.count_jokers();

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
		(self.get_type(), &self.0).cmp(&(other.get_type(), &other.0))
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl FromStr for Hand {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut iter = s.chars().map(Card::try_from);
		Ok(Self([
			iter.next().ok_or(())??,
			iter.next().ok_or(())??,
			iter.next().ok_or(())??,
			iter.next().ok_or(())??,
			iter.next().ok_or(())??,
		]))
	}
}

fn parse_hand_and_bet(s: &str) -> Result<(Hand, i32), ()> {
	let (hand, bet) = s.split_once(' ').ok_or(())?;
	let hand = hand.parse::<Hand>()?;
	let bet = bet.parse::<i32>().map_err(|_| ())?;
	Ok((hand, bet))
}

fn get_answer1(input: &str) -> impl Display {
	let hands = input
		.lines()
		.map(parse_hand_and_bet)
		.collect::<Result<BinaryHeap<_>, ()>>()
		.unwrap()
		.into_sorted_vec();

	hands
		.into_iter()
		.enumerate()
		.map(|(n, (_, bet))| (n + 1) as i32 * bet)
		.sum::<i32>()
}

fn get_answer2(input: &str) -> impl Display {
	let hands = input
		.lines()
		.map(parse_hand_and_bet)
		.map(|hb| hb.map(|(h, b)| (h.jokerify(), b)))
		.collect::<Result<BinaryHeap<_>, ()>>()
		.unwrap()
		.into_sorted_vec();

	hands
		.into_iter()
		.enumerate()
		.map(|(n, (_, bet))| (n + 1) as i32 * bet)
		.sum::<i32>()
}
