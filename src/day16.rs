use std::{fs, collections::BTreeSet, collections::HashSet, thread};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
	UP,
	DOWN,
	RIGHT,
	LEFT
}
const STACK_SIZE: usize = 4 * 1024 * 1024;

pub fn day16() {
	// Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

fn run() {
	let file = fs::read_to_string("input/day16.txt").expect("Should have read file");

	let matrix: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

	let part1 = solve(&matrix, [(0,0)].to_vec(), Direction::RIGHT);

	println!("Day 16 part 1: {}", part1);

	let part2 = solve_part2(&matrix);

	println!("Day 16 part 2: {}", part2);
}

fn solve_part2(matrix: &Vec<Vec<char>>) -> usize {
	let mut top = Vec::new();
	for y in 0..matrix[0].len() {
		top.push((0, y as i32));
	}
	let mut bottom = Vec::new();
	for y in 0..matrix[0].len() {
		bottom.push((matrix.len() as i32-1, y as i32));
	}
	let mut left = Vec::new();
	for x in 0..matrix.len() {
		left.push((x as i32, 0));
	}
	let mut right = Vec::new();
	for x in 0..matrix.len() {
		right.push((x as i32, matrix[0].len() as i32-1));
	}

	let t = solve(matrix, top, Direction::DOWN);
	let b = solve(matrix, bottom, Direction::UP);
	let l = solve(matrix, left, Direction::RIGHT);
	let r = solve(matrix, right, Direction::LEFT);
	
	t.max(b).max(l).max(r)
}

fn solve(matrix: &Vec<Vec<char>>, coords: Vec<(i32, i32)>, d: Direction) -> usize {
	let mut max = 0;
	for coord in coords {
		let mut res = BTreeSet::new();
		lightning(&matrix, coord, d, &mut res);
		let points: HashSet<(usize, usize)> = res.iter().map(|x| x.0).collect();
		let total = points.len();
		if max < total {
			max = total;
		}
	}
	return max;
}

fn lightning(matrix: &Vec<Vec<char>>, (x, y): (i32, i32), mut d: Direction, mem: &mut BTreeSet<((usize, usize), Direction)>) {
	if x < 0 || y < 0 {
		return;
	}
	let (x, y) = (x as usize, y as usize);
	if x >= matrix.len() || y >= matrix[0].len() {
		return;
	}

	if mem.contains(&((x, y), d)) {
		return;
	}
	mem.insert(((x, y), d));

	if matrix[x][y] == '|' && (d == Direction::RIGHT || d == Direction::LEFT) {
		lightning(matrix, next_point((x, y), Direction::UP), Direction::UP, mem);
		lightning(matrix, next_point((x, y), Direction::DOWN), Direction::DOWN, mem);
		return;
	}

	if matrix[x][y] == '-' && (d == Direction::DOWN || d == Direction::UP) {
		lightning(matrix, next_point((x, y), Direction::LEFT), Direction::LEFT, mem);
		lightning(matrix, next_point((x, y), Direction::RIGHT), Direction::RIGHT, mem);
		return;
	}

	if matrix[x][y] == '/' {
		d = match d {
			Direction::DOWN => Direction::LEFT,
			Direction::UP => Direction::RIGHT,
			Direction::LEFT => Direction::DOWN,
			Direction::RIGHT => Direction::UP
		}
	}

	if matrix[x][y] == '\\' {
		d = match d {
			Direction::DOWN => Direction::RIGHT,
			Direction::UP => Direction::LEFT,
			Direction::LEFT => Direction::UP,
			Direction::RIGHT => Direction::DOWN
		}
	}

	lightning(matrix, next_point((x, y), d), d, mem);
}

fn next_point((x, y): (usize, usize), d: Direction) -> (i32, i32) {
	let (x, y) = (x as i32, y as i32);
	return match d {
		Direction::DOWN => (x+1, y),
		Direction::UP => (x-1, y),
		Direction::LEFT => (x, y-1),
		Direction::RIGHT => (x, y+1),
	};
}