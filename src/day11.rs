use std::fs;

use itertools::Itertools;

pub fn day11() {
	let file = fs::read_to_string("input/day11.txt").expect("Should have read file");

	let mut matrix: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

	let original_matrix = matrix.clone();

	let mut x_indx = Vec::new();
	for x in 0..matrix.len() {
		let all_x = matrix[x].iter().all(|x| x == &'.');
		if all_x {
			x_indx.push(x);
		}
	}

	x_indx.reverse();

	for x in x_indx.iter() {
		matrix.insert(*x, matrix[*x].clone());
	}

	let mut y_indx = Vec::new();

	for y in 0..matrix[0].len() {
		let mut dot = true;
		for x in 0..matrix.len() {
			if matrix[x][y] != '.' {
				dot = false;
			}
		}

		if dot {
			y_indx.push(y);
		}
	}

	y_indx.reverse();

	for y in y_indx.iter() {
		for x in 0..matrix.len() {
			matrix[x].insert(*y, '.');
		}
	}
	
	let mut galaxies = Vec::new();

	for x in 0..matrix.len() {
		for y in 0..matrix[0].len() {
			if matrix[x][y] == '#' {
				galaxies.push((x, y));
			}
		}
	}

	let all_galaxy_pairs: Vec<Vec<(usize, usize)>> = galaxies.into_iter().combinations_with_replacement(2).filter(|x| x[0] != x[1]).collect_vec();

	let part1: i32 = all_galaxy_pairs.iter().map(|x| ((x[0].0 as i32) - (x[1].0 as i32)).abs() + ((x[0].1 as i32) - (x[1].1 as i32)).abs()).sum();

	println!("Day 11 part 1: {}", part1);

	let mut galaxies = Vec::new();
	for x in 0..original_matrix.len() {
		for y in 0..original_matrix[0].len() {
			if original_matrix[x][y] == '#' {
				galaxies.push((x, y));
			}
		}
	}

	let space_between = 1000000;

	let galaxies: Vec<(i128, i128)> = galaxies.into_iter().map(|x| {
		let (x, y) = x;
		let x_expanses = x_indx.iter().filter(|x1| **x1 < x).count() as i128;
		let y_expanses = y_indx.iter().filter(|y1| **y1 < y).count() as i128;
		let (x, y) = (x as i128, y as i128);
		((x - x_expanses) + x_expanses * space_between, (y - y_expanses) + y_expanses * space_between)
	}).collect();

	let all_galaxy_pairs: Vec<Vec<(i128, i128)>> = galaxies.into_iter().combinations_with_replacement(2).filter(|x| x[0] != x[1]).collect_vec();

	let part2: i128 = all_galaxy_pairs.iter().map(|x| ((x[0].0 as i128) - (x[1].0 as i128)).abs() + ((x[0].1 as i128) - (x[1].1 as i128)).abs()).sum();
	println!("Day 11 part 2: {}", part2);
}