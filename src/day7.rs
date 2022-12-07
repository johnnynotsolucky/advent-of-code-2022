use std::{ops::Deref, time::Instant};

use nom::{
	bytes::complete::{take, take_until},
	character::complete::space1,
	combinator::opt,
	sequence::tuple,
	IResult,
};

fn main() {
	let input = std::fs::read_to_string("inputs/day7.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

fn part1(input: &str) -> usize {
	Dirs::from(input)
		.iter()
		.map(|item| item.total_size)
		.filter(|size| size < &100_000)
		.sum()
}

fn part2(input: &str) -> usize {
	let dirs = Dirs::from(input);

	const TOTAL_SPACE: usize = 70_000_000;
	const MIN_REQUIRED: usize = 30_000_000;

	let total_used = &dirs.first().expect("Should have root dir").total_size;
	let required_free = MIN_REQUIRED - (TOTAL_SPACE - total_used);

	let mut candidates = dirs
		.iter()
		.map(|item| item.total_size)
		.filter(|size| size > &required_free)
		.collect::<Vec<usize>>();
	candidates.sort_unstable();
	*candidates.first().expect("Should have value")
}

#[derive(Debug)]
struct Dirs {
	inner: Vec<Dir>,
}

impl Deref for Dirs {
	type Target = Vec<Dir>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<'input> From<&'input str> for Dirs {
	fn from(input: &'input str) -> Self {
		let mut dirs: Vec<Dir> = Vec::new();

		let root = Dir {
			parent: None,
			total_size: 0,
		};

		dirs.push(root);

		let mut current_key = 0usize;

		for line in input.lines().skip(1) {
			if line.starts_with('$') {
				match Command::from(&line[2..]) {
					Command::ChangeDir(ChangeDirCommand::MoveUp) => {
						current_key = dirs[current_key].parent.unwrap_or(0);
					}
					Command::ChangeDir(ChangeDirCommand::MoveIn) => {
						let new_idx = dirs.len();

						let move_to_dir = Dir {
							parent: Some(current_key),
							total_size: 0,
						};

						dirs.push(move_to_dir);

						current_key = new_idx;
					}
					Command::List => {}
				}
			} else {
				// Just skip dirs, we create them when we traverse them
				if !line.starts_with("dir") {
					let file = File::from(line);

					let mut parent_idx = Some(current_key);
					while parent_idx.is_some() {
						let parent = dirs.get_mut(parent_idx.unwrap()).expect("Expected parent");
						parent.total_size += file.size;
						parent_idx = parent.parent;
					}
				}
			}
		}

		Self { inner: dirs }
	}
}

#[derive(Debug)]
struct Dir {
	parent: Option<usize>,
	total_size: usize,
}

#[derive(Debug)]
struct File {
	size: usize,
}

impl<'item> From<&'item str> for File {
	fn from(value: &'item str) -> Self {
		let parse_result: IResult<&str, &str> = take_until(" ")(value);
		let (_, size) = parse_result.expect("Expected file details");

		Self {
			size: size.parse().expect("Expected number"),
		}
	}
}

#[derive(Debug)]
enum Command {
	ChangeDir(ChangeDirCommand),
	List,
}

impl<'item> From<&'item str> for Command {
	fn from(value: &'item str) -> Self {
		let parse_result: IResult<&str, (&str, Option<&str>)> =
			tuple((take(2usize), opt(space1)))(value);
		let (value, (command, _)) = parse_result.expect("Expected command");
		match command {
			"cd" => Self::ChangeDir(ChangeDirCommand::from(value)),
			"ls" => Self::List,
			_ => panic!("Unexpected command"),
		}
	}
}

#[derive(Debug)]
enum ChangeDirCommand {
	MoveUp,
	MoveIn,
}

impl<'item> From<&'item str> for ChangeDirCommand {
	fn from(value: &'item str) -> Self {
		match value {
			".." => Self::MoveUp,
			_ => Self::MoveIn,
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(TEST_INPUT), 95437);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(TEST_INPUT), 24933642);
	}
}
