use std::{collections::HashSet, time::Instant};

fn main() {
	let input = std::fs::read_to_string("inputs/day6.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

fn find_marker(input: &str, marker_count: usize) -> usize {
	input.chars().collect::<Vec<_>>()[..]
		.windows(marker_count)
		.enumerate()
		.find(|(_, window)| HashSet::<&char>::from_iter(window.iter()).len() == marker_count)
		.expect("No marker found")
		.0 + marker_count
}

fn part1(input: &str) -> usize {
	find_marker(input, 4)
}

fn part2(input: &str) -> usize {
	find_marker(input, 14)
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: [(&str, usize, usize); 5] = [
		("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
		("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
		("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
		("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
		("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
	];

	#[test]
	fn test_part1() {
		TEST_INPUT.iter().for_each(|(input, expect_end_marker, _)| {
			assert_eq!(part1(*input), *expect_end_marker);
		});
	}

	#[test]
	fn test_part2() {
		TEST_INPUT
			.iter()
			.for_each(|(input, _, expect_start_marker)| {
				assert_eq!(part2(*input), *expect_start_marker);
			});
	}
}
