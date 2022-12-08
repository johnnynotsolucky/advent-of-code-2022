use std::time::Instant;

fn main() {
	let input = std::fs::read_to_string("inputs/day8.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

fn create_matrix(input: &str) -> Vec<Vec<u8>> {
	input
		.lines()
		.map(|line| {
			line.as_bytes()
				.iter()
				.map(|b| atoi::atoi(&[*b]).unwrap())
				.collect()
		})
		.collect::<Vec<Vec<u8>>>()
}

fn is_visible_from<'item>(direction: &mut impl Iterator<Item = &'item u8>, current: &u8) -> bool {
	direction.all(|s| s < current)
}

fn count_til_blocked<'item>(
	direction: &mut impl Iterator<Item = &'item u8>,
	current: &u8,
) -> usize {
	let mut max = 0usize;
	for s in direction {
		if s >= current {
			return max + 1;
		}

		max += 1;
	}

	max
}

fn part1(input: &str) -> usize {
	let matrix = create_matrix(input);

	// So we don't have to keep calling .len() everywhere
	let rows = matrix.len();
	let cols = matrix.first().unwrap().len();

	let mut visible_trees = 0usize;

	let mut row_idx = 1usize;
	for row in &matrix[1..rows - 1] {
		let mut col_idx = 1usize;

		for item in &row[1..cols - 1] {
			if is_visible_from(&mut row[0..col_idx].iter(), item) // Left
				|| is_visible_from(&mut row[col_idx + 1..cols].iter(), item) // Right
				|| is_visible_from(&mut matrix[0..row_idx].iter().map(|s| &s[col_idx]), item) // Top
				|| is_visible_from( // Bottom
					&mut matrix[row_idx + 1..rows].iter().map(|s| &s[col_idx]),
					item,
				) {
				visible_trees += 1;
			}

			col_idx += 1;
		}

		row_idx += 1;
	}

	// Left/right rows + top/bottom columns, with 2 trees removed per column because
	// they're included in the rows.
	visible_trees + rows * 2 + cols * 2 - 4
}

fn part2(input: &str) -> usize {
	let matrix = create_matrix(input);

	let rows = matrix.len();
	let cols = matrix.first().unwrap().len();

	let mut best_view = 0usize;

	let mut row_idx = 1usize;

	for row in &matrix[1..rows - 1] {
		let mut col_idx = 1usize;

		for item in &row[1..cols - 1] {
			let left = count_til_blocked(&mut row[0..col_idx].iter().rev(), item);
			let right = count_til_blocked(&mut row[col_idx + 1..cols].iter(), item);
			let top = count_til_blocked(
				&mut matrix[0..row_idx].iter().map(|s| &s[col_idx]).rev(),
				item,
			);
			let bottom = count_til_blocked(
				&mut matrix[row_idx + 1..rows].iter().map(|s| &s[col_idx]),
				item,
			);

			let total = left * right * top * bottom;

			if best_view < total {
				best_view = total;
			}
			col_idx += 1;
		}

		row_idx += 1;
	}

	best_view
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const TEST_INPUT: &str = r#"30373
25512
65332
33549
35390"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(TEST_INPUT), 21);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(TEST_INPUT), 8);
	}
}
