use std::fs;
use std::cmp::min;

#[derive(Debug)]
struct Range {
	destination_range_start: u64,
	source_range_start: u64,
	range_length: u64
}

#[derive(Debug)]
struct Map {
	name: String,
	ranges: Vec<Range>
}

pub fn day5() {
	let file = fs::read_to_string("input/day5.txt").expect("Should have read file");

	let initial_split: Vec<&str> = file.split("\n\n").collect();

	let mut seeds: Vec<u64> = initial_split[0].split(": ").collect::<Vec<&str>>()[1].split(" ").map(|x| x.parse().unwrap()).collect();

	let mut maps: Vec<Map> = Vec::new();

	for i in 1..initial_split.len() {
		let lines: Vec<&str> = initial_split[i].lines().collect();
		let name: &str = &lines[0][0..lines[0].len()-5];
		let mut ranges: Vec<Range> = Vec::new();
		for j in 1..lines.len() {
			let line = lines[j];
			let split: Vec<u32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
			let range = Range {
				destination_range_start: split[0] as u64,
				source_range_start: split[1] as u64,
				range_length: split[2] as u64
			};
			ranges.push(range);
		}
		maps.push(Map {
			name: name.to_string(),
			ranges: ranges
		});
	}

	for map in &maps {
		let new_seeds = seeds.into_iter().map(|seed| {
			let matching = map.ranges.iter().filter(|x| x.source_range_start <= seed && x.source_range_start+x.range_length >= seed).next();
			return match matching {
				Some(range) => range.destination_range_start + (seed - range.source_range_start),
				None => seed
			};
		}).collect();
		seeds = new_seeds;
	}

	println!("Day 1 part 1: {:?}", seeds.iter().min().unwrap());

	let mut seeds: Vec<(u64, u64)> = initial_split[0].split(": ").collect::<Vec<&str>>()[1].split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u32>>().chunks(2).map(|x| (x[0] as u64, x[1] as u64)).collect();
	
	for map in maps {
		let new_seeds = seeds.into_iter().flat_map(|seed| {
			let mut adds = Vec::new();
			let mut start = seed.0;
			let mut length = seed.1;
			while length > 0 {
				let matching = map.ranges.iter().filter(|x| x.source_range_start <= start && x.source_range_start+x.range_length > start).next();
				match matching {
					Some(range) => {
						let start_diff = start - range.source_range_start;
						let range_covered = min(range.range_length-start_diff, length);
						adds.push((range.destination_range_start+start_diff, range_covered));
						start = start + range_covered;
						length = length - range_covered;
					},
					None => {
						let next = map.ranges.iter().filter(|x| x.source_range_start > start).map(|x| x.source_range_start).min();
						match next {
							Some(nend) => {
								if nend >= start+length {
									adds.push((start, length));
									length = 0;
								} else {
									let range_covered = nend-start;
									adds.push((start, range_covered));
									start = start + range_covered;
									length = length - range_covered;
								}
							},
							None => {
								adds.push((start, length));
								length = 0;
							}
						}
					}
				};
			}
			return adds;
		}).collect();
		seeds = new_seeds;
	}

	println!("Day 1 part 2: {:?}", seeds.iter().map(|x| x.0).min().unwrap());
} // too high 124747783