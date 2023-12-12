use std::fs;
use memoize::memoize;

pub fn day12() {
	let file = fs::read_to_string("input/day12.txt").expect("Should have read file");

	let mut part1 = 0;
	let mut part2 = 0;

	for line in file.lines() {
		let split: Vec<&str> = line.split(" ").collect();
		let nums: Vec<u32> = split[1].split(",").map(|x| x.parse().unwrap()).collect();

		let part2line = format!("{}?{}?{}?{}?{}", split[0], split[0], split[0], split[0], split[0]);
		let part2nums = nums.repeat(5);

		part1 += resolve(split[0].chars().collect(), nums);
		part2 += resolve(part2line.chars().collect(), part2nums);
	}

	println!("Day 12 part 1: {}", part1);
	println!("Day 12 part 2: {}", part2);
}

#[memoize]
fn grouping(mut line: Vec<char>, mut groups: Vec<u32>) -> u64 {
	let group = *groups.first().unwrap() as usize;
	groups.remove(0);

	if line.len() < group {
		return 0;
	}

	let drain: Vec<char> = line.drain(0..group).collect();

	let flag = drain.iter().all(|x| x != &'.');

	if flag {
		if !line.is_empty() {
			if line[0] == '#' {
				return 0;
			}
			line.remove(0);
		}
		return resolve(line, groups);
	}
	else {
		return 0;
	}
}

#[memoize]
fn resolve(mut line: Vec<char>, groups: Vec<u32>) -> u64 {
	if line.is_empty() && groups.is_empty() {
		return 1;
	}

	if line.is_empty() && !groups.is_empty() {
		return 0;
	}

	if !line.is_empty() && groups.is_empty() {
		if line.contains(&'#') {
			return 0;
		}
		return 1;
	}

	if line[0] == '.' {
		line.remove(0);
		return resolve(line, groups);
	}

	if line[0] == '#' {
		return grouping(line, groups);
	}

	if line[0] == '?' {
		let mut n_line = line.clone();
		n_line.remove(0);
		return resolve(n_line, groups.clone()) + grouping(line, groups);
	}

	panic!("Unknown char");
}