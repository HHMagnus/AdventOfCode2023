extern crate itertools;
use std::fs;

use itertools::Itertools;

pub fn day3() {
	let file = fs::read_to_string("input/day3.txt").expect("Should have read file");

	let matrix: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

	let mut day1 = 0;

	let x_max = matrix.len() as i32;
	let y_max = matrix.get(0).unwrap().len() as i32;
	for x in 0..x_max{
		let mut xys: Vec<(i32, i32)> = Vec::new();
		let mut num: Option<i32> = None;
		for y in 0..y_max {
			let pos = matrix[x as usize][y as usize];
			if pos.is_digit(10) {
				let digit = pos.to_digit(10).unwrap() as i32;
				if let Some(st) = num {
					num = Some(st * 10 + digit);
					xys.push((x-1, y));
					xys.push((x+1, y));
				}
				if None == num {
					num = Some(digit);
					xys.push((x-1, y-1));
					xys.push((x, y-1));
					xys.push((x+1, y-1));
					xys.push((x-1, y));
					xys.push((x+1, y));
				}
			} else {
				if let Some(st) = num {
					xys.push((x-1, y));
					xys.push((x, y));
					xys.push((x+1, y));

					let contains_symbol = xys.clone().into_iter()
						.filter(|x| x.0 >= 0 && x.0 < x_max && x.1 >= 0 && x.1 < y_max)
						.any(|xy| matrix[xy.0 as usize][xy.1 as usize] != '.' && !matrix[xy.0 as usize][xy.1 as usize].is_digit(10));
					
					if contains_symbol {
						day1 += st;
					}
					num = None;
					xys.clear();
				}
			}
		}

		if let Some(st) = num {
			let contains_symbol = xys.clone().into_iter()
				.filter(|x| x.0 >= 0 && x.0 < x_max && x.1 >= 0 && x.1 < y_max)
				.any(|xy| matrix[xy.0 as usize][xy.1 as usize] != '.' && !matrix[xy.0 as usize][xy.1 as usize].is_digit(10));
					
			if contains_symbol {
				day1 += st;
			}
		}
	}

	println!("Day 3 part 1: {}", day1);

	let mut gear_nums = Vec::new();

	let x_max = matrix.len() as i32;
	let y_max = matrix.get(0).unwrap().len() as i32;
	for x in 0..x_max {
		let mut xys: Vec<(i32, i32)> = Vec::new();
		let mut num: Option<i32> = None;
		for y in 0..y_max {
			let pos = matrix[x as usize][y as usize];
			if pos.is_digit(10) {
				let digit = pos.to_digit(10).unwrap() as i32;
				if let Some(st) = num {
					num = Some(st * 10 + digit);
					xys.push((x-1, y));
					xys.push((x+1, y));
				}
				if None == num {
					num = Some(digit);
					xys.push((x-1, y-1));
					xys.push((x, y-1));
					xys.push((x+1, y-1));
					xys.push((x-1, y));
					xys.push((x+1, y));
				}
			} else {
				if let Some(st) = num {
					xys.push((x-1, y));
					xys.push((x, y));
					xys.push((x+1, y));

					let gears: Vec<(i32, i32)> = xys.clone().into_iter()
						.filter(|x| x.0 >= 0 && x.0 < x_max && x.1 >= 0 && x.1 < y_max)
						.filter(|xy| matrix[xy.0 as usize][xy.1 as usize] == '*')
						.collect();

					for gear in gears {
						gear_nums.push((gear.0, gear.1, st));
					}
					num = None;
					xys.clear();
				}
			}
		}

		if let Some(st) = num {
			let gears: Vec<(i32, i32)> = xys.clone().into_iter()
				.filter(|x| x.0 >= 0 && x.0 < x_max && x.1 >= 0 && x.1 < y_max)
				.filter(|xy| matrix[xy.0 as usize][xy.1 as usize] == '*')
				.collect();

			for gear in gears {
				gear_nums.push((gear.0, gear.1, st));
			}
		}
	}

	// Sort is needed since "Group by" only groups elements next to eachother
	gear_nums.sort_by_key(|x| (x.0, x.1));

	let day2: i64 = gear_nums
		.iter()
		.group_by(|(x, y, _st)| (x,y))
		.into_iter()
		.map(|(_pos, group)| group.map(|x| x.2).collect())
		.filter(|x: &Vec<i32>| x.len() == 2)
		.map(|x| *x.get(0).unwrap() as i64 * *x.get(1).unwrap() as i64)
		.sum();

	println!("Day 3 part 2: {}", day2);
}