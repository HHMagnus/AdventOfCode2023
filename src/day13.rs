use std::fs;

pub fn day13() {
	let file = fs::read_to_string("input/day13.txt").expect("Should have read file");

	let splits: Vec<&str> = file.split("\n\n").collect();

	let matrices: Vec<Vec<Vec<char>>> = splits.into_iter().map(|x| x.lines().map(|x| x.chars().collect()).collect()).collect();

	let part1: u32 = matrices.iter().map(|x| 100 * horizontal1(x) + vertical1(x)).sum();
	println!("Day 13 part 1: {}", part1);

	let part2: u32 = matrices.iter().map(|x| 100 * horizontal2(x) + vertical2(x)).sum();
	println!("Day 13 part 2: {}", part2);
}

fn vertical1(matrix: &Vec<Vec<char>>) -> u32 {
	for i in 1..matrix[0].len() {
		let mut all = true;
		let mut j = 0;
		while (i as i32 - j as i32 -1) >= 0 && i+j < matrix[0].len() {
			for x in 0..matrix.len() {
				if !matrix[x][i-j-1].eq(&matrix[x][i+j]) {
					all = false;
				}
			}
			j+=1;
		}

		if all {
			return i as u32;
		}
	}
	return 0;
}

fn horizontal1(matrix: &Vec<Vec<char>>) -> u32 {
	for i in 1..matrix.len() {
		let mut all = true;
		let mut j = 0;
		while (i as i32 -j as i32 -1) >= 0 && i+j < matrix.len() {
			if !matrix[i-j-1].eq(&matrix[i+j]) {
				all = false;
			}
			j += 1;
		}

		if all {
			return i as u32;
		}
	}
	return 0;
}

fn vertical2(matrix: &Vec<Vec<char>>) -> u32 {
	for i in 1..matrix[0].len() {
		let mut all = 0;
		let mut j = 0;
		while (i as i32 - j as i32 -1) >= 0 && i+j < matrix[0].len() {
			for x in 0..matrix.len() {
				if !matrix[x][i-j-1].eq(&matrix[x][i+j]) {
					all += 1;
				}
			}
			j+=1;
		}

		if all == 1 {
			return i as u32;
		}
	}
	return 0;
}

fn horizontal2(matrix: &Vec<Vec<char>>) -> u32 {
	for i in 1..matrix.len() {
		let mut all = 0;
		let mut j = 0;
		while (i as i32 -j as i32 -1) >= 0 && i+j < matrix.len() {
			for y in 0..matrix[0].len() {
				if !matrix[i-j-1][y].eq(&matrix[i+j][y]) {
					all += 1;
				}
			}
			
			j += 1;
		}

		if all == 1{
			return i as u32;
		}
	}
	return 0;
}