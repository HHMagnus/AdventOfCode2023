use std::fs;

pub fn day15() {
	let file = fs::read_to_string("input/day15.txt").expect("Should have read file").replace("\n", "");

	let strs: Vec<&str> = file.split(",").collect();

	//println!("{}", hash("HASH"));

	let part1: u32 = strs.clone().into_iter().map(hash).sum();
	println!("Day 15 part 1: {}", part1);

	let mut lenses = vec![Vec::<(&str, u32)>::new(); 256];

	for str in strs.into_iter() {
		let without_minus = str.replace("-", "");
		let label = without_minus.split("=").next().unwrap();
		let index = hash(label) as usize;
		if str.ends_with("-") {
			let val = str.split("-").next().unwrap();
			if let Some(i) = lenses[index].clone().iter().enumerate().find(|&x| (*x.1).0 == val) {
				lenses[index].remove(i.0);
			}
		}
		if str.contains("=") {
			let s: Vec<&str> = str.split("=").collect();
			let val = s[0];
			let num: u32 = s[1].parse().unwrap();
			
			if let Some(i) = lenses[index].clone().iter().enumerate().find(|&x| (*x.1).0 == val) {
				lenses[index][i.0] = (val, num);
			} else {
				lenses[index].push((val, num));
			}
		}
	}

	let mut part2 = 0;

	for j in 0..256 {
		for (i, x) in lenses[j].clone().into_iter().enumerate() {
			let power = (i+1) as u32 * (j+1) as u32 * x.1;
			part2 += power;
		}
	}

	println!("Day 15 part 2: {}", part2);
}

fn hash(s: &str) -> u32 {
	let mut value = 0;

	for c in s.chars() {
		if c == '\n' { continue }
		value += if c.is_lowercase() { c.to_ascii_lowercase() as u32 } else { c.to_ascii_uppercase() as u32 };
		value *= 17;
		value %= 256;
	}

	return value;
}