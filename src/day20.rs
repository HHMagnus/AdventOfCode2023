use std::{fs, collections::{HashMap, VecDeque}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
	High,
	Low
}

#[derive(Debug)]
enum Module {
	FlipFlip(Vec<String>),
	Conjuction(Vec<String>),
	Normal(Vec<String>)
}

pub fn day20() {
	let file = fs::read_to_string("input/day20.txt").expect("Should have read file");

	let modules: HashMap<String, Module> = file.lines().map(|x| {
		let s: Vec<&str> = x.split(" -> ").collect();
		let dest: Vec<String> = s[1].split(", ").map(|x| x.to_string()).collect();
		if s[0].starts_with("%") {
			(s[0].strip_prefix(|_| true).unwrap().to_string(), Module::FlipFlip(dest))
		} else if s[0].starts_with("&") {
			(s[0].strip_prefix(|_| true).unwrap().to_string(), Module::Conjuction(dest))
		} else {
			(s[0].to_string(), Module::Normal(dest))
		}
	}).collect();

	let mut conjunctions: HashMap<String, HashMap<String, Pulse>> = modules.iter().filter(|x| {
		match x.1 {
			Module::Conjuction(_) => true,
			_ => false
		}
	}).map(|x| {
		let dest_map: HashMap<String, Pulse> = modules.iter().filter(|y| {
			let dests = match y.1 {
				Module::Conjuction(dests) => dests,
				Module::FlipFlip(dests) => dests,
				Module::Normal(dests) => dests,
			};
			dests.contains(x.0)
		}).map(|x| (x.0.clone(), Pulse::Low)).collect();
		(x.0.clone(), dest_map)
	}).collect();

	let mut flip_flops: HashMap<String, bool> = modules.iter().filter(|x| {
		match x.1 {
			Module::FlipFlip(_) => true,
			_ => false
		}
	}).map(|x| {
		(x.0.clone(), false)
	}).collect();

	let mut lows = 0;
	let mut highs = 0;

	let mut pulses = VecDeque::new();

	let mut part1 = None;

	let mut i = 0_i64;

	let mut rx_input: HashMap<String, Vec<i64>> = modules.iter().filter(|x| match x.1 {
		Module::Conjuction(dests) => dests,
		Module::FlipFlip(dests) => dests,
		Module::Normal(dests) => dests
	}.contains(&"vr".to_string())).map(|x| (x.0.clone(), Vec::new())).collect();

	while i < 1000 || rx_input.values().all(|x| x.len() < 3) {
		pulses.push_back(("broadcaster", Pulse::Low, "button"));

		while pulses.len() > 0 {
			let next = pulses.pop_front().unwrap();
	
			match next.1 {
				Pulse::High => highs += 1,
				Pulse::Low => lows += 1
			}

			if rx_input.contains_key(next.2) && next.1 == Pulse::High {
				let mut ds = rx_input.get(next.2).unwrap().clone();
				ds.push(i);
				rx_input.insert(next.2.to_string(), ds);
			}
			
			if !modules.contains_key(next.0) {
				continue;
			}

			let module = modules.get(next.0).unwrap();
	
			match module {
				Module::FlipFlip(dests) => {
					let flipper = flip_flops.get(next.0).unwrap();
					match (next.1, flipper) {
						(Pulse::High, _) => {},
						(Pulse::Low, false) => {
							flip_flops.insert(next.0.to_string(), true);
							for dest in dests {
								pulses.push_back((dest, Pulse::High, next.0));
							}
						},
						(Pulse::Low, true) => {
							flip_flops.insert(next.0.to_string(), false);
							for dest in dests {
								pulses.push_back((dest, Pulse::Low, next.0));
							}
						},
					}
				},
				Module::Conjuction(dests) => {
					let conj = conjunctions.get_mut(next.0).unwrap();
					conj.insert(next.2.to_string(), next.1);
					let all_high = conj.values().all(|&x| x == Pulse::High);
					let pulse = match all_high {
						true => Pulse::Low,
						false => Pulse::High
					};
					for dest in dests {
						pulses.push_back((dest, pulse, next.0));
					}
				},
				Module::Normal(dests) => {
					for dest in dests {
						pulses.push_back((dest, next.1, next.0));
					}
				}
			};
		}

		i += 1;

		if i == 1000 {
			part1 = Some(lows*highs);
		}
	}

	let part2 = lcm(&rx_input.values().map(|x| x[1]-x[0]).collect::<Vec<i64>>());

	println!("Day 20 part 1: {:?}", part1.unwrap());
	println!("Day 20 part 2: {:?}", part2);
}

// returns the least common multiple of n numbers
// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}