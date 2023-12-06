use std::fs;
use regex::Regex;
use std::iter::zip;

pub fn day6() {
	let file = fs::read_to_string("input/day6.txt").expect("Should have read file");
	let reg = Regex::new(r"\s+").unwrap();
	let split: Vec<&str> = file.split("\n").collect();
	let mut time: Vec<&str> = reg.split(split[0]).collect();
	time.remove(0);
	let time: Vec<u32> = time.iter().map(|x| x.parse().unwrap()).collect();
	let mut distance: Vec<&str> = reg.split(split[1]).collect();
	distance.remove(0);
	let distance: Vec<u32> = distance.iter().map(|x| x.parse().unwrap()).collect();

	let input: Vec<(u32, u32)> = zip(time, distance).collect();

	let mut part1 = 1;

	for i in input {
		let time = i.0;
		let distance = i.1;

		let mut ways = 0;
		for i in 0..time {
			let f = (time-i) * i;
			if f > distance {
				ways+=1;
			}
		}

		part1 *= ways;
	}

	println!("Day 6 part 1: {}", part1);

	let time = reg.replace_all(split[0], "").to_string().split(":").collect::<Vec<&str>>()[1].parse::<u64>().unwrap();
	let distance = reg.replace_all(split[1], "").to_string().split(":").collect::<Vec<&str>>()[1].parse::<u64>().unwrap();

	let mut part2 = 0;

	for i in 0..time {
		let f = (time-i) * i;
		if f > distance {
			part2+=1;
		}
	}
	
	println!("Day 6 part 2: {}", part2);
}