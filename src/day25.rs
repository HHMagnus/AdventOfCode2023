use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub fn day25() {
	let file = fs::read_to_string("input/day25.txt").expect("Should have read file");

	let input: Vec<(&str, Vec<&str>)> = file.lines().map(|x| {
		let split: Vec<&str> = x.split(": ").collect();
		let name = split[0];
		let list: Vec<&str> = split[1].split(" ").collect();
		(name, list)
	}).collect();

	let mut nodes = HashMap::new();

	for i in input {
		let node = nodes.entry(i.0).or_insert(HashSet::new());

		for &x in &i.1 {
			node.insert(x);
		}

		for &x in &i.1 {
			let rev = nodes.entry(x).or_insert(HashSet::new());
			rev.insert(i.0);
		}
	}
	
	let mut usage = HashMap::new();

	for &start in nodes.keys() {
		let mut queue = VecDeque::new();
		queue.push_back(start);

		let mut seen = HashSet::new();
		seen.insert(start);

		while let Some(next) = queue.pop_front() {
			for &to in &nodes[next] {
				if seen.contains(to) { continue;}
				seen.insert(to);

				let edge = if to < next { [to, next] } else { [next, to] };
				let used = usage.entry(edge).or_insert(0);
				*used += 1;
				
				queue.push_back(to);
			}
		}
	}

	let mut freq: Vec<_> = usage.iter().filter(|x| x.0[0] != x.0[1]).collect();
	freq.sort_by_key(|&e| *e.1);
	freq.reverse();

	let cut: Vec<_> = freq.iter().take(3).map(|&x| *x.0).collect();
	let start = cut[2][1];
	let mut size = 1;

	let mut queue = VecDeque::new();
	queue.push_back(start);

	let mut seen = HashSet::new();
	seen.insert(start);

	while let Some(next) = queue.pop_front() {
		for &to in &nodes[next] {

			let edge = if to < next { [to, next] } else { [next, to] };

			if cut.contains(&edge) {
				continue;
			}

			if !seen.contains(to) {
				seen.insert(to);
				queue.push_back(to);
				size += 1;
			}
		}
	}

	println!("Day 25 part 1: {}", size * (nodes.len() - size));

	println!("Day 25 part 2: Press the button!");
}