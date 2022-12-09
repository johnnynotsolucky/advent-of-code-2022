use std::{
	cmp::Ordering,
	collections::HashSet,
	ops::{Add, Sub},
	time::Instant,
};

use nom::{
	bytes::complete::take,
	character::complete::{char, digit1},
	sequence::{preceded, tuple},
	IResult,
};

fn main() {
	let input = std::fs::read_to_string("inputs/day9.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl From<&str> for Direction {
	fn from(value: &str) -> Self {
		match value {
			"U" => Self::Up,
			"D" => Self::Down,
			"L" => Self::Left,
			"R" => Self::Right,
			_ => panic!("Invalid move character"),
		}
	}
}

impl From<Direction> for Coord {
	fn from(value: Direction) -> Self {
		let t = match value {
			Direction::Up => (0, 1),
			Direction::Down => (0, -1),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0),
		};

		t.into()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord(isize, isize);

impl Add for Coord {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl Sub for Coord {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0, self.1 - rhs.1)
	}
}

impl From<(isize, isize)> for Coord {
	fn from(value: (isize, isize)) -> Self {
		Self(value.0, value.1)
	}
}

impl Coord {
	fn move_adjacent(&mut self, other: &Coord, visited: Option<&mut HashSet<Coord>>) {
		let diff = *other - *self;
		if diff.0.abs() > 1 || diff.1.abs() > 1 {
			let next_x = match 0.cmp(&diff.0) {
				Ordering::Less => 1,
				Ordering::Greater => -1,
				_ => 0,
			};

			let next_y = match 0.cmp(&diff.1) {
				Ordering::Less => 1,
				Ordering::Greater => -1,
				_ => 0,
			};

			*self = *self + Coord::from((next_x, next_y));

			if let Some(visited) = visited {
				visited.insert(*self);
			}
		}
	}
}

fn parse_move(line: &str) -> (Direction, isize) {
	let result: IResult<&str, (&str, &str)> =
		tuple((take(1usize), preceded(char(' '), digit1)))(line);
	let (_, (direction, amount)) = result.expect("Invalid move");

	(
		Direction::from(direction),
		amount.parse().expect("Invalid value"),
	)
}

fn part1(input: &str) -> usize {
	let mut visited: HashSet<Coord> = HashSet::new();
	visited.insert(Coord::from((0, 0)));

	let mut tail = Coord(0, 0);
	let mut head = Coord(0, 0);

	for line in input.lines() {
		let (direction, amount) = parse_move(line);

		let step = Coord::from(direction);
		for _ in 0..amount {
			head = head + step;
			tail.move_adjacent(&head, Some(&mut visited));
		}
	}

	visited.len()
}

fn part2(input: &str) -> usize {
	const COUNT: usize = 10;
	let mut visited: HashSet<Coord> = HashSet::new();
	visited.insert(Coord::from((0, 0)));

	let mut rope: Vec<Coord> = std::iter::repeat(Coord::from((0, 0))).take(COUNT).collect();

	for line in input.lines() {
		let (direction, amount) = parse_move(line);

		let step = Coord::from(direction);
		// let head = rope.get_mut(0).unwrap();
		for _ in 0..amount {
			// move head
			let head = rope.get_mut(0).unwrap();
			*head = *head + step;

			for idx in 1..COUNT {
				let prev = rope[idx - 1];

				let visited = if idx == COUNT - 1 {
					Some(&mut visited)
				} else {
					None
				};
				rope[idx].move_adjacent(&prev, visited);
			}
		}
	}

	visited.len()
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT_PART1: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(TEST_INPUT_PART1), 13);
	}

	const TEST_INPUT_PART2: [(&str, usize); 2] = [
		(
			r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#,
			1,
		),
		(
			r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#,
			36,
		),
	];
	#[test]
	fn test_part2() {
		for (input, count) in TEST_INPUT_PART2 {
			assert_eq!(part2(input), count);
		}
	}
}
