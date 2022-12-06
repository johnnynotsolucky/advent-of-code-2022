use std::{collections::BTreeSet, time::Instant};

fn main() {
	let input = std::fs::read_to_string("inputs/day3.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

fn part1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let split_len = line.len() / 2;
			let left: BTreeSet<char> = line[..split_len].chars().collect();
			let right: BTreeSet<char> = line[split_len..].chars().collect();

			left.intersection(&right)
				.map(|common| {
					let priority = if common.is_uppercase() {
						*common as u8 - b'A' + 27
					} else {
						*common as u8 - b'a' + 1
					};

					priority as u32
				})
				.next()
				.expect("Should have a value")
		})
		.sum()
}

fn part2(input: &str) -> u32 {
	let lines: Vec<_> = input.lines().collect();
	lines
		.chunks(3)
		.map(|chunk| {
			let badge = chunk
				.iter()
				.map(|line| line.chars().collect::<BTreeSet<char>>())
				.fold(None, |acc, line| match acc {
					None => Some(line),
					Some(acc) => Some(acc.intersection(&line).cloned().collect()),
				})
				.expect("Should have set")
				.first()
				.cloned()
				.expect("Should have a value");

			let priority = if badge.is_uppercase() {
				badge as u8 - b'A' + 27
			} else {
				badge as u8 - b'a' + 1
			};

			priority as u32
		})
		.sum()
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(&TEST_INPUT), 157);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(&TEST_INPUT), 70);
	}
}
