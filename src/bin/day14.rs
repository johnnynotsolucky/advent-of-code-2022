use std::{collections::HashSet, time::Instant};

use nom::{
	bytes::complete::tag,
	character::complete::{char, digit1},
	combinator::map_opt,
	multi::separated_list1,
	sequence::separated_pair,
	IResult,
};

fn main() {
	let input = std::fs::read_to_string("inputs/day14.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

fn parse_row(val: &[u8]) -> IResult<&[u8], Vec<(usize, usize)>> {
	separated_list1(tag(" -> "), parse_pair)(val)
}

fn parse_pair(line: &[u8]) -> IResult<&[u8], (usize, usize)> {
	separated_pair(parse_int, char(','), parse_int)(line)
}

fn parse_int(val: &[u8]) -> IResult<&[u8], usize> {
	map_opt(digit1, atoi::atoi)(val)
}

fn scan_cave(input: &str) -> (usize, HashSet<(usize, usize)>) {
	let mut lowest = 0;
	let mut cave = HashSet::new();

	for line in input.lines() {
		for window in parse_row(line.as_bytes()).unwrap().1[..].windows(2) {
			let [from, to]: &[_; 2] = window.try_into().unwrap();

			lowest = lowest.max(from.1.max(to.1));

			if from.0 != to.0 {
				let min = from.0.min(to.0);
				let max = from.0.max(to.0);
				cave.extend((min..max + 1).map(|x| (x, to.1)))
			} else {
				let min = from.1.min(to.1);
				let max = from.1.max(to.1);
				cave.extend((min..max + 1).map(|y| (to.0, y)))
			}
		}
	}

	(lowest, cave)
}

fn flood_cave(
	cave: &mut HashSet<(usize, usize)>,
	source: (usize, usize),
	target: usize,
	break_on_target: bool,
) -> usize {
	std::iter::repeat(())
		.take_while(|_| {
			let mut curr_pos = source;
			'sand: loop {
				if curr_pos.1 == target {
					if break_on_target {
						break;
					} else {
						return false;
					}
				}

				let directions = [
					(curr_pos.0, curr_pos.1 + 1),
					(curr_pos.0 - 1, curr_pos.1 + 1),
					(curr_pos.0 + 1, curr_pos.1 + 1),
				];

				for direction in directions {
					if cave.get(&direction).is_none() {
						curr_pos = direction;
						continue 'sand;
					}
				}

				break;
			}

			if curr_pos == source {
				return false;
			}

			cave.insert(curr_pos);

			true
		})
		.count()
}

fn part1(input: &str) -> usize {
	let source = (500usize, 0usize);
	let (lowest, mut cave) = scan_cave(input);

	flood_cave(&mut cave, source, lowest, false)
}

fn part2(input: &str) -> usize {
	let source = (500usize, 0usize);
	let (lowest, mut cave) = scan_cave(input);

	flood_cave(&mut cave, source, lowest + 1, true) + 1
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(TEST_INPUT), 24);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(TEST_INPUT), 93);
	}
}
