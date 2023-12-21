use std::fs;

use itertools::Itertools;

pub fn day21() {
	let file = fs::read_to_string("input/day21.txt").expect("Should have read file");

	let matrix: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

	let start = matrix.iter().enumerate().find(|x| x.1.contains(&'S')).map(|x| (x.0, x.1.iter().position(|&y| y == 'S').unwrap())).unwrap();

	let part1 = solve_part1((start.0 as i32, start.1 as i32), &matrix);

	println!("Day 21 part 1: {}", part1);

	let part2 = solve_part2(26501365, (start.0 as i32, start.1 as i32), &matrix);

	println!("Day 21 part 2: {}", part2);
}

fn solve_part1(pos: (i32, i32), matrix: &Vec<Vec<char>>) -> u32 {
	let x_max = matrix.len() as i32;
	let y_max = matrix.len() as i32;

	let mut positions = Vec::new();

	positions.push(pos);

	for _ in 0..64 {
		positions = positions.iter().flat_map(|&pos| {
			[
				(pos.0 +1, pos.1),
				(pos.0 -1, pos.1),
				(pos.0, pos.1 +1),
				(pos.0, pos.1 -1)
			].to_vec().into_iter().filter(|&x| {
				if x.0 < 0 || x.1 < 0 || x.0 >= x_max || x.1 >= y_max {
					return false;
				}

				if matrix[x.0 as usize][x.1 as usize] == '#' {
					return false;
				}

				return true;
			}).collect::<Vec<(i32, i32)>>()
		}).unique().collect();
	}

	// This was used to view the square that is produced
	for x in 0..x_max {
		for y in 0..y_max {
			print!("{}", if positions.contains(&(x, y)) { 'O' } else { matrix[x as usize][y as usize] });
		}
		println!("");
	}

	return positions.len() as u32;
}

fn solve_part2(steps: u64, pos: (i32, i32), matrix: &Vec<Vec<char>>) -> u64 {
	let x_max = matrix.len() as i32;
	let y_max = matrix.len() as i32;

	let mapsized = matrix.len() as i32;
	let s = pos.0;

	let mut positions = Vec::new();

	positions.push(pos);

	let mut last = 0;
	let mut diff = 0 as i32;
	let mut ddiff = 0 as i32;

	let mut i = 1;

	loop {
		positions = positions.iter().flat_map(|&pos| {
			[
				(pos.0 +1, pos.1),
				(pos.0 -1, pos.1),
				(pos.0, pos.1 +1),
				(pos.0, pos.1 -1)
			].to_vec().into_iter().filter(|&x| {
				let mut a = x.0;
				while a < 0 {
					a += x_max;
				}
				while a >= x_max {
					a -= x_max;
				}
				let mut b = x.1;
				while b < 0 {
					b += y_max;
				}
				while b >= y_max {
					b -= y_max;
				}
				if matrix[a as usize][b as usize] == '#' {
					return false;
				}

				return true;
			}).collect::<Vec<(i32, i32)>>()
		}).unique().collect();
		if i % mapsized == s {
			let curr = positions.len() as u32;
			let d = curr as i32 - last as i32;
			let dd = d-diff;
			println!("{}, {}, {}, {}", i, curr, d, dd);
			last = curr;
			diff = d;
			if ddiff == dd {
				break;
			}
			ddiff = dd;
		}

		i += 1;
	}

	let mut curr = positions.len() as u64;
	let mut diff = diff as u64;
	let ddiff = ddiff as u64;
	
	while i < steps as i32 {
		diff += ddiff;
		curr += diff;
		i += mapsized;
	}
	println!("{}, {}, {}, {}", i, curr, diff, ddiff);

	return curr;
}