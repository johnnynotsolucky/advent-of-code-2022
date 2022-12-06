use std::time::Instant;

use nom::{
	bytes::complete::tag,
	character::complete::digit1,
	combinator::map_res,
	sequence::{preceded, tuple},
	IResult,
};

fn main() {
	let input = std::fs::read_to_string("inputs/day5.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

fn parse_move(line: &str) -> IResult<&str, (usize, usize, usize)> {
	tuple((
		preceded(tag("move "), map_res(digit1, str::parse)),
		preceded(tag(" from "), map_res(digit1, str::parse)),
		preceded(tag(" to "), map_res(digit1, str::parse)),
	))(line)
}

fn part1(input: &str) -> String {
	let mut deck = Vec::new();
	let mut building_deck = true;
	for line in input.lines() {
		if line.is_empty() || line.starts_with(" 1") {
			building_deck = false;
			continue;
		}

		if building_deck {
			for (stack, storage_crate) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
				if deck.len() < stack + 1 {
					deck.push(vec![]);
				}

				let storage_crate = storage_crate[1];
				if storage_crate != ' ' {
					let stack = deck.get_mut(stack).expect("Should have a stack already");
					stack.push(storage_crate);
				}
			}
		} else {
			let (_, (count, from, to)) = parse_move(line).expect("Invalid move");
			let from_stack = deck.get_mut(from - 1).expect("Invalid `from` stack");
			let mut lifted = from_stack.drain(..count).rev().collect::<Vec<_>>();

			let to_stack = deck.get_mut(to - 1).expect("Invalid `to` stack");
			lifted.append(to_stack);
			*to_stack = lifted;
		}
	}

	deck.iter()
		.map(|stack| stack.first().expect("Should have a crate"))
		.collect::<String>()
}

fn part2(input: &str) -> String {
	let mut deck = Vec::new();
	let mut building_deck = true;
	for line in input.lines() {
		if line.is_empty() || line.starts_with(" 1") {
			building_deck = false;
			continue;
		}

		if building_deck {
			for (stack, storage_crate) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
				if deck.len() < stack + 1 {
					deck.push(vec![]);
				}

				let storage_crate = storage_crate[1];
				if storage_crate != ' ' {
					let stack = deck.get_mut(stack).expect("Should have a stack already");
					stack.push(storage_crate);
				}
			}
		} else {
			let (_, (count, from, to)) = parse_move(line).expect("Invalid move");
			let from_stack = deck.get_mut(from - 1).expect("Invalid `from` stack");
			let mut lifted = from_stack.drain(..count).collect::<Vec<_>>();

			let to_stack = deck.get_mut(to - 1).expect("Invalid `to` stack");
			lifted.append(to_stack);
			*to_stack = lifted;
		}
	}

	deck.iter()
		.map(|stack| stack.first().expect("Should have a crate"))
		.collect::<String>()
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(&TEST_INPUT), "CMZ");
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(&TEST_INPUT), "MCD");
	}
}
