use std::time::Instant;

use nom::{
	character::complete::{char, digit1},
	sequence::separated_pair,
	IResult,
};

fn main() {
	let input = std::fs::read_to_string("inputs/day4.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

struct Assignment {
	min: u32,
	max: u32,
}

impl Assignment {
	fn is_subset_of(&self, other: &Assignment) -> bool {
		self.min >= other.min && self.max <= other.max
	}

	fn overlaps_with(&self, other: &Assignment) -> bool {
		(self.min >= other.min && self.min <= other.max)
			|| (self.max <= other.max && self.max >= other.min)
	}
}

impl From<(&str, &str)> for Assignment {
	fn from(value: (&str, &str)) -> Self {
		Self {
			min: value.0.parse().expect("Expected min value"),
			max: value.1.parse().expect("Expected max value"),
		}
	}
}

type ParseResult<'l> = IResult<&'l str, ((&'l str, &'l str), (&'l str, &'l str))>;

fn parse_match(line: &str) -> ParseResult {
	let range_parser = || separated_pair(digit1, char('-'), digit1);
	separated_pair(range_parser(), char(','), range_parser())(line)
}

fn part1(input: &str) -> usize {
	input
		.lines()
		.map(|line| {
			let assignment_match = parse_match(line).expect("Invalid match");

			(
				Assignment::from(assignment_match.1 .0),
				Assignment::from(assignment_match.1 .1),
			)
		})
		.filter(|(a, b)| a.is_subset_of(b) || b.is_subset_of(a))
		.count()
}

fn part2(input: &str) -> usize {
	input
		.lines()
		.map(|line| {
			let assignment_match = parse_match(line).expect("Invalid match");

			(
				Assignment::from(assignment_match.1 .0),
				Assignment::from(assignment_match.1 .1),
			)
		})
		.filter(|(a, b)| a.overlaps_with(b) || b.overlaps_with(a))
		.count()
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(&TEST_INPUT), 2);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(&TEST_INPUT), 4);
	}
}
