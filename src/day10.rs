use std::{fs, collections::VecDeque};

use itertools::Itertools;
use colored::Colorize;

#[derive(Debug, Clone, Copy)]
enum Direction {
	NORTH,
	EAST,
	WEST,
	SOUTH
}

pub fn day10() {
	let file = fs::read_to_string("input/day10.txt").expect("Should have read file");

	let matrix: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

	let start = matrix.iter().enumerate().find_or_first(|x| x.1.contains(&'S')).map(|x| (x.0, x.1.iter().enumerate().find_or_first(|y| y.1 == &'S').unwrap().0)).unwrap();

	// Start has to be manually set!
	let mut curr = (start.0-1, start.1);
	let mut dir = Direction::SOUTH;

	let mut steps = 1;

	let mut pipe = Vec::new();
	pipe.push(start);
	let mut left = Vec::new();
	let mut right = Vec::new();
	while matrix[curr.0][curr.1] != 'S' {
		pipe.push(curr);

		let uppsize = (curr.0 as i32, curr.1 as i32);
		match (dir, matrix[curr.0][curr.1]) {
			(Direction::NORTH, '|') => {
				left.push((uppsize.0, uppsize.1+1));
				right.push((uppsize.0, uppsize.1-1));
			},
			(Direction::SOUTH, '|') => {
				right.push((uppsize.0, uppsize.1+1));
				left.push((uppsize.0, uppsize.1-1));
			},
			(Direction::EAST, '-') => {
				right.push((uppsize.0-1, uppsize.1));
				left.push((uppsize.0+1, uppsize.1));
			},
			(Direction::WEST, '-') => {
				left.push((uppsize.0-1, uppsize.1));
				right.push((uppsize.0+1, uppsize.1));
			},
			(Direction::NORTH, 'L') => {
				right.push((uppsize.0, uppsize.1-1));
				right.push((uppsize.0+1, uppsize.1));
			},
			(Direction::EAST, 'L') => {
				left.push((uppsize.0, uppsize.1-1));
				left.push((uppsize.0+1, uppsize.1));
			},
			(Direction::NORTH, 'J') => {
				left.push((uppsize.0, uppsize.1+1));
				left.push((uppsize.0+1, uppsize.1));
			},
			(Direction::WEST, 'J') => {
				right.push((uppsize.0, uppsize.1+1));
				right.push((uppsize.0+1, uppsize.1));
			},
			(Direction::SOUTH, '7') => {
				right.push((uppsize.0, uppsize.1+1));
				right.push((uppsize.0-1, uppsize.1));
			},
			(Direction::WEST, '7') => {
				left.push((uppsize.0, uppsize.1+1));
				left.push((uppsize.0-1, uppsize.1));
			},
			(Direction::SOUTH, 'F') => {
				left.push((uppsize.0, uppsize.1-1));
				left.push((uppsize.0-1, uppsize.1));
			},
			(Direction::EAST, 'F') => {
				right.push((uppsize.0, uppsize.1-1));
				right.push((uppsize.0-1, uppsize.1));
			},
			_ => panic!("{:?}, {:?}, {:?}", curr, matrix[curr.0][curr.1], dir)
		};

		dir = match (dir, matrix[curr.0][curr.1]) {
			(Direction::NORTH, '|') => Direction::SOUTH,
			(Direction::SOUTH, '|') => Direction::NORTH,
			(Direction::EAST, '-') => Direction::WEST,
			(Direction::WEST, '-') => Direction::EAST,
			(Direction::NORTH, 'L') => Direction::EAST,
			(Direction::EAST, 'L') => Direction::NORTH,
			(Direction::NORTH, 'J') => Direction::WEST,
			(Direction::WEST, 'J') => Direction::NORTH,
			(Direction::SOUTH, '7') => Direction::WEST,
			(Direction::WEST, '7') => Direction::SOUTH,
			(Direction::SOUTH, 'F') => Direction::EAST,
			(Direction::EAST, 'F') => Direction::SOUTH,
			_ => panic!("{:?}, {:?}, {:?}", curr, matrix[curr.0][curr.1], dir)
		};

		curr = match dir {
			Direction::NORTH => (curr.0-1, curr.1),
			Direction::SOUTH => (curr.0+1, curr.1),
			Direction::EAST => (curr.0, curr.1+1),
			Direction::WEST => (curr.0, curr.1-1)
		};

		dir = match dir {
			Direction::NORTH => Direction::SOUTH,
			Direction::SOUTH => Direction::NORTH,
			Direction::EAST => Direction::WEST,
			Direction::WEST => Direction::EAST,
		};
		steps += 1;
	}

	let part1 = steps/2;
	println!("Day 10 part 1: {}", part1);

	left = left
		.into_iter()
		.filter(|x| x.0 >= 0 && x.0 < (matrix.len() as i32) && x.1 >= 0 && x.1 < (matrix[0].len() as i32))
		.filter(|x| !pipe.contains(&(x.0 as usize, x.1 as usize)))
		.unique()
		.collect::<Vec<(i32, i32)>>();
	right = right
		.into_iter()
		.filter(|x| x.0 >= 0 && x.0 < (matrix.len() as i32) && x.1 >= 0 && x.1 < (matrix[0].len() as i32))
		.filter(|x| !pipe.contains(&(x.0 as usize, x.1 as usize)))
		.unique()
		.collect::<Vec<(i32, i32)>>();

	let left_outside = left.iter().any(|x| x.0 == 0 || x.1 == 0);
	let right_outside = right.iter().any(|x| x.0 == 0 || x.1 == 0);

	if left_outside && right_outside {
		println!("{:?}", left);
		println!("{:?}", right);
		panic!("Both sides are outside!");
	}

	let mut empties = VecDeque::new();

	for x in 0..matrix.len() {
		for y in 0..matrix[0].len() {
			let lrc = (x as i32, y as i32);
			if left.contains(&lrc) || right.contains(&lrc) || pipe.contains(&(x, y)) {
				continue;
			}
			empties.push_front((x, y));
		}
	}

	while empties.len() > 0 {
		let (x, y) = empties.pop_front().unwrap();
		let lrc = (x as i32, y as i32);
		if x > 0 {
			let neb = ((x-1) as i32, y as i32);
			if left.contains(&neb) {
				left.push(lrc);
				continue;
			}
			if right.contains(&neb) {
				right.push(lrc);
				continue;
			}
		}
		if y > 0 {
			let neb = (x as i32, (y-1) as i32);
			if left.contains(&neb) {
				left.push(lrc);
				continue;
			}
			if right.contains(&neb) {
				right.push(lrc);
				continue;
			}
		}
		if x < matrix.len()-1 {
			let neb = ((x+1) as i32, y as i32);
			if left.contains(&neb) {
				left.push(lrc);
				continue;
			}
			if right.contains(&neb) {
				right.push(lrc);
				continue;
			}
		}
		if y < matrix[0].len()-1 {
			let neb = (x as i32, (y+1) as i32);
			if left.contains(&neb) {
				left.push(lrc);
				continue;
			}
			if right.contains(&neb) {
				right.push(lrc);
				continue;
			}
		}
		empties.push_back((x, y));
	}

	for x in 0..matrix.len() {
		for y in 0..matrix[0].len() {
			let mut c = matrix[x][y];
			if left.contains(&(x as i32, y as i32)) {
				c = if left_outside { 'O' } else { 'I' };
			}
			if right.contains(&(x as i32, y as i32)) {
				c = if right_outside { 'O' } else { 'I' };
			}
			print!("{}", if c == 'O' || c == 'I' { c.to_string().bold() } else { c.to_string().normal() });
		}
		println!("");
	}

	let part2 = if right_outside { left.len() } else { right.len() };

	println!("Day 10 part 2: {}", part2);
}