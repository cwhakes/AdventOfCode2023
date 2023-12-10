#[cfg(test)]
mod test;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::fs;

fn main() {
	let buf = fs::read_to_string("input/10/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dir {
	North,
	East,
	South,
	West,
}

impl Dir {
	fn reverse(&self) -> Self {
		match self {
			Dir::North => Dir::South,
			Dir::East => Dir::West,
			Dir::South => Dir::North,
			Dir::West => Dir::East,
		}
	}

	fn get_offset(&self) -> (i32, i32) {
		match self {
			Dir::North => (0, -2),
			Dir::East => (2, 0),
			Dir::South => (0, 2),
			Dir::West => (-2, 0),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pipe(Dir, Dir);

impl Pipe {
	fn new(c: char) -> Option<Self> {
		match c {
			'|' => Some(Self(Dir::North, Dir::South)),
			'-' => Some(Self(Dir::East, Dir::West)),
			'L' => Some(Self(Dir::North, Dir::East)),
			'J' => Some(Self(Dir::North, Dir::West)),
			'7' => Some(Self(Dir::South, Dir::West)),
			'F' => Some(Self(Dir::South, Dir::East)),
			_ => None,
		}
	}

	fn follow(&self, from_dir: Dir) -> Option<Dir> {
		if self.0 == from_dir.reverse() {
			Some(self.1.clone())
		} else if self.1 == from_dir.reverse() {
			Some(self.0.clone())
		} else {
			None
		}
	}
}

struct Field(HashMap<(i32, i32), Pipe>);

impl Field {
	fn new(s: &str) -> (Self, (i32, i32)) {
		let mut field = HashMap::new();
		let mut start_position = (-1, -1);
		for (y, line) in s.lines().enumerate() {
			for (x, c) in line.chars().enumerate() {
				if c == 'S' {
					start_position = (2 * x as i32, 2 * y as i32);
				}
				if let Some(pipe) = Pipe::new(c) {
					field.insert((2 * x as i32, 2 * y as i32), pipe);
				}
			}
		}

		let start_pipe = match [Dir::North, Dir::East, Dir::South, Dir::West].map(|d| {
			let x = start_position.0 + d.get_offset().0;
			let y = start_position.1 + d.get_offset().1;
			if let Some(pipe) = field.get(&(x, y)) {
				pipe.follow(d).is_some()
			} else {
				false
			}
		}) {
			[true, false, true, false] => Pipe(Dir::North, Dir::South),
			[false, true, false, true] => Pipe(Dir::East, Dir::West),
			[true, true, false, false] => Pipe(Dir::North, Dir::East),
			[true, false, false, true] => Pipe(Dir::North, Dir::West),
			[false, false, true, true] => Pipe(Dir::South, Dir::West),
			[false, true, true, false] => Pipe(Dir::South, Dir::East),
			_ => panic!(),
		};
		field.insert(start_position, start_pipe);

		(Self(field), start_position)
	}

	fn bounds(&self) -> ((i32, i32), (i32, i32)) {
		(
			(
				*self.0.keys().map(|(x, _)| x).min().unwrap(),
				*self.0.keys().map(|(x, _)| x).max().unwrap(),
			),
			(
				*self.0.keys().map(|(_, y)| y).min().unwrap(),
				*self.0.keys().map(|(_, y)| y).max().unwrap(),
			),
		)
	}

	fn next(&self, pos: (i32, i32), dir: Dir) -> ((i32, i32), Dir) {
		let (x, y) = (pos.0 + dir.get_offset().0, pos.1 + dir.get_offset().1);
		let next_dir = self.0.get(&(x, y)).unwrap().follow(dir).unwrap();
		((x, y), next_dir)
	}

	fn find_cycle(&self, start_pos: (i32, i32)) -> usize {
		let mut dir = self.0.get(&start_pos).unwrap().0.clone();
		let mut pos = start_pos;
		for i in 1.. {
			(pos, dir) = self.next(pos, dir);
			if pos == start_pos {
				return i;
			}
		}
		unreachable!()
	}

	fn find_area(&self, start_pos: (i32, i32)) -> usize {
		let mut dir = self.0.get(&start_pos).unwrap().0.clone();
		let mut pos = start_pos;

		let ((x_min, x_max), (y_min, y_max)) = self.bounds();
		let mut area = ((x_min - 1)..=(x_max + 1))
			.flat_map(|x| ((y_min - 1)..=(y_max + 1)).map(move |y| (x, y)))
			.collect::<HashSet<_>>();
		let area_len = area
			.iter()
			.filter(|(x, y)| x % 2 == 0 && y % 2 == 0)
			.count();

		let mut i = 0;
		let loop_len = loop {
			i += 1;
			area.remove(&pos);
			area.remove(&(
				pos.0 + dir.get_offset().0 / 2,
				pos.1 + dir.get_offset().1 / 2,
			));
			(pos, dir) = self.next(pos, dir);
			if pos == start_pos {
				break i;
			}
		};

		let mut visited = HashSet::new();
		let mut queue = VecDeque::new();
		queue.push_back((x_min - 1, y_min - 1));
		while let Some((x, y)) = queue.pop_front() {
			if visited.insert((x, y)) {
				for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
					let (x, y) = (x + dir.get_offset().0 / 2, y + dir.get_offset().1 / 2);
					if area.contains(&(x, y)) {
						queue.push_back((x, y));
					}
				}
			}
		}
		let visited_len = visited
			.iter()
			.filter(|(x, y)| x % 2 == 0 && y % 2 == 0)
			.count();

		dbg!(area_len) - dbg!(visited_len) - dbg!(loop_len)
	}
}

fn get_answer1(input: &str) -> impl Display {
	let (field, start_pos) = Field::new(input);
	field.find_cycle(start_pos) / 2
}

fn get_answer2(input: &str) -> impl Display {
	let (field, start_pos) = Field::new(input);
	field.find_area(start_pos)
}
