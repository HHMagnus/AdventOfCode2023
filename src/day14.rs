use std::{fs, collections::HashMap};

pub fn day14() {
	let file = fs::read_to_string("input/day14.txt").expect("Should have read file");

	let mut matrix: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

	north(&mut matrix);
	
	let part1: usize = matrix.iter().map(|x| x.iter().filter(|x| x == &&'O').count()).enumerate().map(|x| (matrix.len()-x.0) * x.1).sum();
	println!("Day 14 part 1: {}", part1);

	west(&mut matrix);
	south(&mut matrix);
	east(&mut matrix);

	let cycle = 1000000000;

	let mut prevs = HashMap::new();

	for i in 2..cycle {
		north(&mut matrix);
		west(&mut matrix);
		south(&mut matrix);
		east(&mut matrix);

		if let Some(x) = prevs.insert(matrix.clone(), i) {
			if (1000000000 - i) % (i - x) == 0 {
				break;
			}
		}
	}

	let part2: usize = matrix.iter().map(|x| x.iter().filter(|x| x == &&'O').count()).enumerate().map(|x| (matrix.len()-x.0) * x.1).sum();
	println!("Day 14 part 2: {}", part2);

	for x in 0..matrix.len() {
		for y in 0..matrix[0].len() {
			print!("{}", matrix[x][y]);
		}
		println!("");
	}
}

fn north(matrix: &mut Vec<Vec<char>>) {
	for x in 1..matrix.len() {
		for y in 0..matrix[0].len() {
			if matrix[x][y] == 'O' {
				let mut u = x;
				while u > 0 && matrix[u-1][y] == '.' {
					u -= 1;
				}

				if u != x {
					matrix[u][y] = 'O';
					matrix[x][y] = '.';
				}
			}
		}
	}
}


fn south(matrix: &mut Vec<Vec<char>>) {
	for sx in 0..matrix.len() {
		let x = matrix.len()-1-sx;
		for y in 0..matrix[0].len() {
			if matrix[x][y] == 'O' {
				let mut u = x;
				while u < matrix.len()-1 && matrix[u+1][y] == '.' {
					u += 1;
				}

				if u != x {
					matrix[u][y] = 'O';
					matrix[x][y] = '.';
				}
			}
		}
	}
}

fn west(matrix: &mut Vec<Vec<char>>) {
	for x in 0..matrix.len() {
		for y in 0..matrix[0].len() {
			if matrix[x][y] == 'O' {
				let mut u = y;
				while u > 0 && matrix[x][u-1] == '.' {
					u -= 1;
				}

				if u != y {
					matrix[x][u] = 'O';
					matrix[x][y] = '.';
				}
			}
		}
	}
}

fn east(matrix: &mut Vec<Vec<char>>) {
	for x in 0..matrix.len() {
		for ey in 0..matrix[0].len() {
			let y = matrix[0].len() -1 - ey;
			if matrix[x][y] == 'O' {
				let mut u = y;
				while u < matrix[y].len()-1 && matrix[x][u+1] == '.' {
					u += 1;
				}

				if u != y {
					matrix[x][u] = 'O';
					matrix[x][y] = '.';
				}
			}
		}
	}
}