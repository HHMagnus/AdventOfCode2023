use std::fs;

pub fn day9() {
	let file = fs::read_to_string("input/day9.txt").expect("Should have read file");

	let histories: Vec<Vec<i64>> = file.lines().map(|line| line.split(" ").map(|num| num.parse::<i64>().unwrap()).collect()).collect();
	let mut part1 = 0;
	let mut part2 = 0;

	for history in histories.iter() {
		let mut ends = Vec::new();
		let mut starts = Vec::new();
		let mut vals = history.clone();
		ends.push(*vals.last().unwrap());
		starts.push(*vals.first().unwrap());
		while !vals.iter().all(|x| x == &0) {
			let nd: Vec<i64> = vals.windows(2).map(|x| x[1] - x[0]).collect();
			ends.push(*nd.last().unwrap());
			starts.push(*nd.first().unwrap());
			vals = nd;
		}

		ends.reverse();

		let mut next = 0;
		for x in ends {
			next += x;
		}
		part1 += next;

		starts.reverse();

		let mut prev = 0;
		for x in starts {
			prev = x - prev;
		}
		part2 += prev;
	}

	println!("Day 9 part 1: {}", part1);
	println!("Day 9 part 2: {}", part2);
}