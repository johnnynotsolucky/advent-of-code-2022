use nom::{
	character::complete::char, character::complete::one_of, sequence::separated_pair, IResult,
};

fn main() {
	let input = std::fs::read_to_string("inputs/day2.txt").expect("Input file should exist");
	let part1 = part1(&input);
	let part2 = part2(&input);

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}

const SEPARATOR: char = ' ';

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl From<char> for Shape {
	fn from(value: char) -> Self {
		match value {
			'A' | 'X' => Self::Rock,
			'B' | 'Y' => Self::Paper,
			'C' | 'Z' => Self::Scissors,
			_ => panic!("Invalid input"),
		}
	}
}

enum Play {
	Lose,
	Draw,
	Win,
}

impl Play {
	fn shape(&self, opponent_shape: Shape) -> Shape {
		match self {
			Self::Lose => match opponent_shape {
				Shape::Rock => Shape::Scissors,
				Shape::Paper => Shape::Rock,
				Shape::Scissors => Shape::Paper,
			},
			Self::Draw => opponent_shape,
			Self::Win => match opponent_shape {
				Shape::Rock => Shape::Paper,
				Shape::Paper => Shape::Scissors,
				Shape::Scissors => Shape::Rock,
			},
		}
	}
}

impl From<char> for Play {
	fn from(value: char) -> Self {
		match value {
			'X' => Self::Lose,
			'Y' => Self::Draw,
			'Z' => Self::Win,
			_ => panic!("Invalid input"),
		}
	}
}

fn parse_match(line: &str) -> IResult<&str, (char, char)> {
	separated_pair(one_of("ABC"), char(SEPARATOR), one_of("XYZ"))(line)
}

fn score(left: Shape, right: Shape) -> u32 {
	let match_score = if left == right {
		3
	} else {
		match (&left, &right) {
			(Shape::Rock, Shape::Paper)
			| (Shape::Paper, Shape::Scissors)
			| (Shape::Scissors, Shape::Rock) => 6,
			_ => 0,
		}
	};

	match_score + right as u32
}

fn part1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let parsed_match = parse_match(line).expect("Invalid match");
			let (left, right): (Shape, Shape) =
				(parsed_match.1 .0.into(), parsed_match.1 .1.into());
			score(left, right)
		})
		.sum()
}

fn part2(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let parsed_match = parse_match(line).expect("Invalid match");
			let (left, play): (Shape, Play) = (parsed_match.1 .0.into(), parsed_match.1 .1.into());

			let right = play.shape(left);

			score(left, right)
		})
		.sum()
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"A Y
B X
C Z
"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(&TEST_INPUT), 15);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(&TEST_INPUT), 12);
	}
}
