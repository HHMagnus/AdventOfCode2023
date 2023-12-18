use std::fs;

use itertools::Itertools;

enum Direction {
	Up,
	Down,
	Right,
	Left
}

pub fn day18() {
	let file = fs::read_to_string("input/day18.txt").expect("Should have read file");

	let xs: Vec<(Direction, i64)> = file.lines().map(|x| {
		let x: Vec<&str> = x.split(" ").collect();
		let dir = match x[0] {
			"U" => Direction::Up,
			"D" => Direction::Down,
			"R" => Direction::Right,
			"L" => Direction::Left,
			_ => panic!("Wrong encoding"),
		};
		let num: i64 = x[1].parse().unwrap();
		(dir, num)
	}).collect();

	let part1 = solve(xs);

	println!("Day 18 part 1: {:?}", part1);

	let inst: Vec<(Direction, i64)> = file.lines().map(|x| {
		let x = x.split(" ").last().unwrap().replace("(", "").replace(")", "");
		let end = x.chars().last().unwrap();
		let dir = match end {
			'0' => Direction::Right,
			'1' => Direction::Down,
			'2' => Direction::Left,
			'3' => Direction::Up,
			_ => panic!("Not encoded correct. {}", x)
		};
		let x = &x[1..x.len()-1];
		let num = i64::from_str_radix(x, 16).unwrap();
		(dir, num)
	}).collect();

	let part2 = solve(inst);

	println!("Day 18 part 2: {:?}", part2);
}

fn solve(instructions: Vec<(Direction, i64)>) -> i64 {
	let mut position = (0,0);
	let mut positions = Vec::new();

	positions.push(position);

	let mut circumference = 0;
	
	for x in instructions {
		let n: i64 = x.1;
		position = match x.0 {
			Direction::Up => (position.0-n, position.1),
			Direction::Down => (position.0+n, position.1),
			Direction::Right => (position.0, position.1+n),
			Direction::Left => (position.0, position.1-n),
			_ => panic!("Not right"),
		};
		circumference += n;
		positions.push(position);
	}

	positions.iter().tuple_windows().map(|((x1, y1), (x2, y2))| (x1 * y2) - (y1 * x2)).sum::<i64>().abs()/2 + circumference/2 + 1
}
