use std::{
	collections::{BTreeMap, BTreeSet},
	time::Instant,
};

fn main() {
	let input = std::fs::read_to_string("inputs/day11.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

#[derive(Debug)]
struct Monkey<'input> {
	items: Vec<usize>,
	operator: &'input str,
	operation_value: Option<usize>,
	test: usize,
	true_cond: usize,
	false_cond: usize,
	inspected: usize,
}

impl<'input> From<&[&'input str]> for Monkey<'input> {
	fn from(lines: &[&'input str]) -> Self {
		let items = lines[0].split(": ").collect::<Vec<_>>()[1]
			.split(", ")
			.map(|i| i.parse::<usize>().unwrap())
			.collect::<Vec<_>>();

		let op = lines[1].split("old ").collect::<Vec<_>>()[1]
			.split(' ')
			.collect::<Vec<_>>();
		let operator = op[0];
		let operation_value = match op[1] {
			"old" => None,
			value => Some(value.parse::<usize>().unwrap()),
		};

		let test = lines[2].split("by ").collect::<Vec<_>>()[1]
			.parse::<usize>()
			.unwrap();
		let true_cond = parse_condition(lines[3]);
		let false_cond = parse_condition(lines[4]);

		Monkey {
			items,
			operator,
			operation_value,
			test,
			true_cond,
			false_cond,
			inspected: 0,
		}
	}
}

fn parse_condition(line: &str) -> usize {
	line.split("monkey ").collect::<Vec<_>>()[1]
		.parse::<usize>()
		.unwrap()
}

fn create_monkey_map(input: &str) -> BTreeMap<usize, Monkey> {
	let mut monkeys = BTreeMap::new();
	for monkey in input.split("\n\n") {
		let lines = monkey.split('\n').collect::<Vec<_>>();
		let monkey_id = lines[0]
			.replace("Monkey ", "")
			.replace(':', "")
			.parse::<usize>()
			.unwrap();
		monkeys.insert(monkey_id, Monkey::from(&lines[1..]));
	}

	monkeys
}

fn part1(input: &str) -> usize {
	let mut monkeys = create_monkey_map(input);
	let monkey_ids = monkeys.keys().copied().collect::<Vec<_>>();

	for _ in 0..20 {
		for monkey_id in &monkey_ids {
			let items = monkeys
				.get_mut(monkey_id)
				.unwrap()
				.items
				.drain(..)
				.collect::<Vec<_>>();
			for item in items {
				let (worry_level, to_monkey) = {
					let monkey = monkeys.get_mut(monkey_id).unwrap();
					let operation_value = monkey.operation_value.unwrap_or(item);
					let worry_level = match monkey.operator {
						"*" => operation_value * item,
						"+" => operation_value + item,
						_ => panic!("Unexpected operator"),
					} / 3;

					let to_monkey = if worry_level % monkey.test == 0 {
						monkey.true_cond
					} else {
						monkey.false_cond
					};

					monkey.inspected += 1;

					(worry_level, to_monkey)
				};

				monkeys.get_mut(&to_monkey).unwrap().items.push(worry_level);
			}
		}
	}

	monkeys
		.values()
		.map(|monkey| monkey.inspected)
		.collect::<BTreeSet<_>>()
		.iter()
		.rev()
		.take(2)
		.product::<usize>()
}

fn part2(input: &str) -> usize {
	let mut monkeys = create_monkey_map(input);
	let monkey_ids = monkeys.keys().copied().collect::<Vec<_>>();
	let divisor = monkeys
		.values()
		.map(|monkey| monkey.test)
		.product::<usize>();

	for _ in 0..10000 {
		for monkey_id in &monkey_ids {
			let items = monkeys
				.get_mut(monkey_id)
				.unwrap()
				.items
				.drain(..)
				.collect::<Vec<_>>();
			for item in items {
				let (worry_level, to_monkey) = {
					let monkey = monkeys.get_mut(monkey_id).unwrap();
					let operation_value = monkey.operation_value.unwrap_or(item);
					let worry_level = match monkey.operator {
						"*" => operation_value * item,
						"+" => operation_value + item,
						_ => panic!("Unexpected operator"),
					} % divisor;

					let to_monkey = if worry_level % monkey.test == 0 {
						monkey.true_cond
					} else {
						monkey.false_cond
					};

					monkey.inspected += 1;

					(worry_level, to_monkey)
				};

				monkeys.get_mut(&to_monkey).unwrap().items.push(worry_level);
			}
		}
	}

	monkeys
		.values()
		.map(|monkey| monkey.inspected)
		.collect::<BTreeSet<_>>()
		.iter()
		.rev()
		.take(2)
		.product::<usize>()
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(TEST_INPUT), 10605);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(TEST_INPUT), 2713310158);
	}
}
