use std::time::Instant;

fn main() {
	let input = std::fs::read_to_string("inputs/day10.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2 ({part2_elapsed}): \n\n{part2} ");
}

fn parse_line(line: &str) -> Option<isize> {
	let parts = &line.split(' ').collect::<Vec<_>>()[..];
	if parts.len() == 2 {
		Some(parts[1].parse().expect("Invalid value"))
	} else {
		None
	}
}

fn part1(input: &str) -> isize {
	let (_, _, s) = input
		.lines()
		.fold((0, 1, 0), |(mut cs, mut x, mut s), line| {
			let (cycles, value) = match parse_line(line) {
				Some(value) => (2, value),
				None => (1, 0),
			};

			for cycle in 0..cycles {
				cs += 1;

				if (cs + 20) % 40 == 0 && cs <= 220 {
					s += cs * x;
				}

				if cycle == 1 {
					x += value;
				}
			}

			(cs, x, s)
		});

	s
}

fn part2(input: &str) -> String {
	let mut crt = std::iter::repeat(std::iter::repeat('.').take(40).collect::<Vec<_>>())
		.take(6)
		.collect::<Vec<_>>();

	let mut row = 0usize;
	let mut col = 0usize;
	let mut sprite_pos = 1;

	for line in input.lines() {
		let (cycles, value) = match parse_line(line) {
			Some(value) => (2, value),
			None => (1, 0),
		};

		for cycle in 0..cycles {
			if (sprite_pos..sprite_pos + 3).contains(&(col as isize + 1)) {
				let pixel = crt.get_mut(row).unwrap().get_mut(col).unwrap();
				*pixel = '#';
			}

			col += 1;

			if col == 40 {
				row += 1;
				col = 0;
			}

			if cycle == 1 {
				sprite_pos += value;
			}
		}
	}

	crt.iter()
		.map(|cols| cols.iter().collect::<String>())
		.collect::<Vec<_>>()
		.join("\n")
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(TEST_INPUT), 13140);
	}

	#[test]
	fn test_part2() {
		assert_eq!(
			part2(TEST_INPUT),
			r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
				.to_owned()
		);
	}
}
