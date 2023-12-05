use std::fs;
use regex::Regex;

pub fn day4() {
	let file = fs::read_to_string("input/day4.txt").expect("Should have read file");
	let mut part1 = 0;
	let mut part2_multiplier = vec![1; 200];
	let mut part2 = 0;
	for (i, line) in file.lines().enumerate() {
		let r1 = Regex::new(r":\s+").unwrap();
		let spl: Vec<&str> = r1.split(line).collect();
		let r2: Regex = Regex::new(r" \|\s+").unwrap();
		let sp: Vec<&str> = r2.split(spl.get(1).unwrap()).collect();
		let regx = Regex::new(r"\s+").unwrap();
		let winners: Vec<u32> = regx.split(sp.get(0).unwrap()).map(|x| x.parse::<u32>().unwrap()).collect();
		let cards: Vec<u32> = regx.split(sp.get(1).unwrap()).map(|x| x.parse::<u32>().unwrap()).collect();

		let num = cards.iter().filter(|x| winners.contains(x)).count();

		if num > 0 {
			let mut x = 1;

			for j in i..i+num {
				part2_multiplier[j+1] += part2_multiplier[i];
			}

			for _i in 1..num {
				x += x;
			}

			part1 += x;
		}

		part2 += part2_multiplier[i];
	}

	println!("Day 1 part 1: {}", part1);
	println!("Day 1 part 2: {}", part2);
}