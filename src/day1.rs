fn main() {
	let input = std::fs::read_to_string("inputs/day1.txt").expect("Input file should exist");
	let elves = parse_elves(&input);

	println!("{}", part1(&elves));
	println!("{}", part2(elves));
}

type Calories = u32;

fn parse_elves(input: &str) -> Vec<Calories> {
	input.lines().fold(vec![], |mut elves, line| {
		let elf = if line.is_empty() {
			Calories::default()
		} else {
			let calories = elves.pop().unwrap_or_else(Calories::default);
			calories + line.parse::<Calories>().expect("Expected calory count")
		};

		elves.push(elf);

		elves
	})
}

fn part1<'c>(elves: &'c Vec<Calories>) -> &'c Calories {
	elves.iter().max().expect("Should not be an empty Vec")
}

fn part2(mut elves: Vec<Calories>) -> Calories {
	// Sort in reverse
	elves.sort_unstable_by(|a, b| b.cmp(a));
	// Fetch the top 3 elves and return the sum of their calories
	elves.iter().take(3).map(|elf| *elf).sum()
}

#[cfg(test)]
mod test {
	use crate::{parse_elves, part1, part2};

	const TEST_INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(&parse_elves(&TEST_INPUT)), &24000);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(parse_elves(&TEST_INPUT)), 45000);
	}
}
