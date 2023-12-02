use std::{fs, collections::HashMap};

pub fn day1() {
	let file = fs::read_to_string("input/day1.txt").expect("Should have read file");
	let day1p1:u32 = file.lines().map(|line| {
		let split =line.chars().filter_map(|x| x.to_digit(10).to_owned()).collect::<Vec<u32>>();
		format!("{}{}", split.first().unwrap(), split.last().unwrap()).to_string().parse::<u32>().unwrap()
	}).sum();
	println!("Day 1 part 1: {:?}", day1p1);
	
	let mut map:HashMap<&str, u32> = HashMap::new();
	map.insert("one", 1);
	map.insert("two", 2);
	map.insert("three", 3);
	map.insert("four", 4);
	map.insert("five", 5);
	map.insert("six", 6);
	map.insert("seven", 7);
	map.insert("eight", 8);
	map.insert("nine", 9);

	let day1p2:Vec<u32> = file.lines().map(|line| {
		let mut vec:Vec<u32> = Vec::new();
		let mut i = 0;
		let line_length = line.len();
		while i < line_length {
			for x in &map {
				if line_length >= i+x.0.len() && &&line[i..i+x.0.len()] == x.0 {
					vec.push(x.1.to_owned());
					i+=1;
					continue;
				}
			}

			match line.to_string().chars().nth(i).map(|x| x.to_digit(10)) {
				Some(Some(x)) => vec.push(x),
				Some(None) => (),
				None => ()
			}

			i+=1;
		}
		format!("{}{}", vec.first().unwrap(), vec.last().unwrap()).to_string().parse::<u32>().unwrap()
	}).collect();
	println!("Day 1 part 2: {:?}", day1p2.into_iter().sum::<u32>());
}