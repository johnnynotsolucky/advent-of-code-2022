use std::{
	collections::{BTreeMap, BinaryHeap},
	time::Instant,
};

fn main() {
	let input = std::fs::read_to_string("inputs/day12.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

#[derive(Debug, PartialEq, Eq)]
struct State<'h> {
	steps: usize,
	node: &'h Node,
}

impl<'h> Ord for State<'h> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other
			.steps
			.cmp(&self.steps)
			.then_with(|| other.node.position.cmp(&self.node.position))
	}
}

impl<'h> PartialOrd for State<'h> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
	elevation: isize,
	position: Position,
}

impl Node {
	fn neighbor<'h>(
		&self,
		direction: &Direction,
		heightmap: &'h BTreeMap<Position, Node>,
	) -> Option<&'h Node> {
		let position: Option<Position> = match direction {
			Direction::Left => self
				.position
				.x
				.checked_sub(1)
				.map(|x| (x, self.position.y).into()),
			Direction::Right => Some((self.position.x + 1, self.position.y).into()),
			Direction::Up => self
				.position
				.y
				.checked_sub(1)
				.map(|y| (self.position.x, y).into()),
			Direction::Down => Some((self.position.x, self.position.y + 1).into()),
		};

		position
			.map(|position| heightmap.get(&position))
			.unwrap_or(None)
	}
}

impl<'input> From<(&'input u8, Position)> for Node {
	fn from((elevation, position): (&'input u8, Position)) -> Self {
		Self {
			elevation: (elevation - b'a') as isize,
			position,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
	x: usize,
	y: usize,
}

impl From<(usize, usize)> for Position {
	fn from((x, y): (usize, usize)) -> Self {
		Position { x, y }
	}
}

enum Direction {
	Left,
	Right,
	Up,
	Down,
}

const DIRECTIONS: &[Direction; 4] = &[
	Direction::Left,
	Direction::Right,
	Direction::Up,
	Direction::Down,
];

fn initialize(
	input: &str,
) -> (
	Position,
	Position,
	BTreeMap<Position, usize>,
	BTreeMap<Position, Node>,
) {
	let mut start_node = None;
	let mut end_node = None;

	let mut steps = BTreeMap::new();

	let mut heightmap: BTreeMap<Position, Node> = BTreeMap::new();
	input
		.as_bytes()
		.split(|b| *b == b'\n')
		.enumerate()
		.for_each(|(y, line)| {
			line.iter().enumerate().for_each(|(x, node)| {
				let node = match node {
					b'S' => {
						let position = (x, y).into();
						let node = Node::from((&b'a', position));
						start_node = Some(position);
						node
					}
					b'E' => {
						let position = (x, y).into();
						let node = Node::from((&b'z', position));
						end_node = Some(position);
						node
					}
					node => Node::from((node, (x, y).into())),
				};

				steps.insert(node.position, usize::MAX);
				heightmap.insert(node.position, node);
			});
		});

	(start_node.unwrap(), end_node.unwrap(), steps, heightmap)
}

fn part1(input: &str) -> usize {
	// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Using_a_priority_queue
	let (start, end, mut steps, heightmap) = initialize(input);
	let mut heap = BinaryHeap::new();
	heap.push(State {
		steps: 0,
		node: heightmap.get(&start).unwrap(),
	});

	*steps.get_mut(&start).unwrap() = 0;

	while let Some(state) = heap.pop() {
		if state.node.position == end {
			break;
		}

		if state.steps > steps[&state.node.position] {
			continue;
		}

		for dir in DIRECTIONS {
			if let Some(neighbor) = state.node.neighbor(dir, &heightmap) {
				if neighbor.elevation - state.node.elevation <= 1 {
					let next_cost = state.steps + 1;

					if next_cost < *steps.get(&neighbor.position).unwrap() {
						heap.push(State {
							steps: next_cost,
							node: neighbor,
						});

						*steps.get_mut(&neighbor.position).unwrap() = next_cost;
					}
				}
			}
		}
	}

	*steps.get(&end).unwrap()
}

fn part2(input: &str) -> usize {
	// Work backwards, track all paths to 0
	let (_, start, mut steps, heightmap) = initialize(input);
	let mut heap = BinaryHeap::new();
	heap.push(State {
		steps: 0,
		node: heightmap.get(&start).unwrap(),
	});

	let mut shortest_routes = Vec::new();

	*steps.get_mut(&start).unwrap() = 0;

	while let Some(state) = heap.pop() {
		if state.node.elevation == 0 {
			shortest_routes.push(state.steps);
			continue;
		}

		if state.steps > steps[&state.node.position] {
			continue;
		}

		for dir in DIRECTIONS {
			if let Some(neighbor) = state.node.neighbor(dir, &heightmap) {
				if state.node.elevation - neighbor.elevation <= 1 {
					let next_cost = state.steps + 1;

					if next_cost < steps[&neighbor.position] {
						heap.push(State {
							steps: next_cost,
							node: neighbor,
						});

						*steps.get_mut(&neighbor.position).unwrap() = next_cost;
					}
				}
			}
		}
	}

	shortest_routes.sort_unstable();
	*shortest_routes.first().unwrap()
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(TEST_INPUT), 31);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(TEST_INPUT), 29);
	}
}
