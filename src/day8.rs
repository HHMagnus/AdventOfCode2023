use core::panic;
use std::{fs, collections::HashMap};

pub fn day8() {
	let file = fs::read_to_string("input/day8.txt").expect("Should have read file");

	let lines: Vec<&str> = file.lines().collect();

	let path: Vec<char> = lines[0].chars().collect();

	let mut nodes= HashMap::new();
	
	for i in 2..lines.len() {
		let line = lines[i];

		let split: Vec<&str> = line.split(" = (").collect();
		
		let start = split[0];

		let ends = split[1];

		let ends_split: Vec<&str> = ends.split(", ").collect();

		let left = ends_split[0];

		let right = ends_split[1].split(")").collect::<Vec<&str>>()[0];

		nodes.insert(start, (left, right));
	}

	let mut steps = 0;

	let mut curr = "AAA";

	let mut path1 = path.iter().cycle();

	while curr != "ZZZ" {
		let mov = path1.next().unwrap();
		let options = nodes.get(curr).unwrap();
		steps += 1;
		if mov == &'L' {
			curr = options.0;
			continue;
		}
		if mov == &'R' {
			curr = options.1;
			continue;
		}
		
		panic!("Unknown move {}", mov);
	}

	println!("Day 7 part 1: {}", steps);

	let starts: Vec<&str> = nodes.keys().filter(|x| x.ends_with('A')).map(|x| *x).collect();

	let mut loopers: Vec<u64> = Vec::new();

	for start in starts {
		let mut known_ends: Vec<(&str, u64)> = Vec::new();

		let mut curr = start;

		let mut path = path.iter().cycle();
		let mut steps = 0;

		loop {
			if curr.ends_with('Z') {
				let known = known_ends.iter().find(|x| x.0 == curr);
				if known.is_some() {
					loopers.push(known.unwrap().1);
					break;
				}
				known_ends.push((curr, steps));
			}

			steps += 1;
			let mov = path.next().unwrap();

			let options = nodes.get(curr).unwrap();
			if mov == &'L' {
				curr = options.0;
				continue;
			}
			if mov == &'R' {
				curr = options.1;
				continue;
			}
		}
	}

	let part2 = lcm(&loopers);

	println!("Day 7 part 2: {}", part2);
}

// returns the least common multiple of n numbers
// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}