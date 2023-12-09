#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

fn main() {
	let buf = fs::read_to_string("input/08/input").unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

#[derive(Clone, Debug)]
enum Dir {
	Left,
	Right,
}

impl From<char> for Dir {
	fn from(value: char) -> Self {
		match value {
			'L' => Self::Left,
			'R' => Self::Right,
			_ => panic!(),
		}
	}
}

#[derive(Clone, Debug)]
struct Node<'a> {
	left: &'a str,
	right: &'a str,
}

impl<'a> Node<'a> {
	fn new(s: &'a str) -> (&'a str, Self) {
		let (name, s) = s.split_once(" = (").unwrap();
		let (left, right) = s.split_once(", ").unwrap();
		let right = right.strip_suffix(')').unwrap();

		(name, Self { left, right })
	}

	fn get(&self, dir: &Dir) -> &'a str {
		match &dir {
			Dir::Left => self.left,
			Dir::Right => self.right,
		}
	}
}

struct Map<'a> {
	dirs: Vec<Dir>,
	nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Map<'a> {
	fn new(s: &'a str) -> Self {
		let (dirs, nodes) = s.split_once("\n\n").unwrap();
		let dirs = dirs.chars().map(Dir::from).collect::<Vec<_>>();
		let nodes = nodes.lines().map(Node::new).collect::<HashMap<_, _>>();
		Self { dirs, nodes }
	}

	fn navigate<F>(&self, from: &str, to: F) -> usize
	where
		F: Fn(&str) -> bool,
	{
		let mut current_node = from;
		for (i, dir) in self.dirs.iter().cycle().enumerate() {
			if to(current_node) {
				return i;
			}
			current_node = self.nodes.get(current_node).unwrap().get(dir);
		}
		unreachable!()
	}
}

fn get_answer1(input: &str) -> impl Display {
	Map::new(input).navigate("AAA", |node| node == "ZZZ")
}

fn get_answer2(input: &str) -> impl Display {
	let map = Map::new(input);
	let starts = map
		.nodes
		.keys()
		.filter(|name| name.ends_with('A'))
		.collect::<Vec<_>>();

	let paths = starts
		.into_iter()
		.map(|s| map.navigate(s, |node| node.ends_with('Z')) as u128)
		.collect::<Vec<_>>();
	paths.into_iter().reduce(num::integer::lcm).unwrap()
}
