use std::{cmp::Ordering, time::Instant};

use nom::{
	branch::alt,
	character::complete::{char, digit1},
	combinator::{map, map_opt},
	multi::separated_list0,
	sequence::{delimited, separated_pair},
	IResult,
};

fn main() {
	let input = std::fs::read_to_string("inputs/day13.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
	Integer(usize),
	List(Vec<Item>),
}

impl Ord for Item {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Item::Integer(left), Item::Integer(right)) => left.cmp(right),
			(Item::Integer(_), Item::List(right)) if right.len() == 1 => self.cmp(&right[0]),
			(Item::Integer(left), Item::List(_)) => {
				Item::List(vec![Item::Integer(*left)]).cmp(other)
			}
			(Item::List(_), Item::Integer(right)) => {
				self.cmp(&Item::List(vec![Item::Integer(*right)]))
			}
			(Item::List(left), Item::List(right)) => left.iter().cmp(right),
		}
	}
}

impl PartialOrd for Item {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn parse_int(val: &[u8]) -> IResult<&[u8], usize> {
	map_opt(digit1, atoi::atoi)(val)
}

fn parse_list(val: &[u8]) -> IResult<&[u8], Vec<Item>> {
	delimited(char('['), separated_list0(char(','), parse_item), char(']'))(val)
}

fn parse_item(val: &[u8]) -> IResult<&[u8], Item> {
	alt((map(parse_int, Item::Integer), map(parse_list, Item::List)))(val)
}

fn parse_pairs(line: &[u8]) -> IResult<&[u8], (Item, Item)> {
	separated_pair(parse_item, char('\n'), parse_item)(line)
}

fn part1(input: &str) -> usize {
	input
		.split("\n\n")
		.enumerate()
		.filter_map(|(idx, pair)| {
			let (_, (left, right)) = parse_pairs(pair.as_bytes()).unwrap();

			match left.cmp(&right) {
				Ordering::Less => Some(idx + 1),
				_ => None,
			}
		})
		.sum::<usize>()
}

fn part2(input: &str) -> usize {
	let dividers = vec![
		Item::List(vec![Item::List(vec![Item::Integer(2)])]),
		Item::List(vec![Item::List(vec![Item::Integer(6)])]),
	];

	let list = input
		.split('\n')
		.filter(|line| !line.is_empty())
		.map(|signal| parse_item(signal.as_bytes()).unwrap().1)
		.filter(|item| item < &dividers[1])
		.collect::<Vec<_>>();

	let pos_a = list.iter().filter(|item| *item < &dividers[0]).count() + 1;
	let pos_b = list.len() + 2;

	pos_a * pos_b
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(TEST_INPUT), 13);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(TEST_INPUT), 140);
	}
}
