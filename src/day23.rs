use std::{fs, thread, collections::{HashSet, HashMap}};

const STACK_SIZE: usize = 4 * 1024 * 1024 * 1024;

pub fn day23() {
	// Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

fn run() {
	let file = fs::read_to_string("input/day23.txt").expect("Should have read file"); 

	let matrix: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

	let start = (0, 1);

	let paths = longest_path_p1(&matrix, HashSet::new(), start);

	let part1 = paths.iter().map(|x| x.len()).max().unwrap();

	println!("Day 23 part 1: {}", part1);

	let part2 = longest_path_p2(&matrix, start, (matrix.len() as i32 -1, matrix[0].len() as i32 -2));

	println!("Day 23 part 2: {}", part2);
}

fn longest_path_p1(matrix: &Vec<Vec<char>>, mut path: HashSet<(i32, i32)>, current: (i32, i32)) -> Vec<HashSet<(i32, i32)>>{
	if current.0 == matrix.len() as i32 -1 && current.1 == matrix[0].len() as i32 -2 {
		return vec![path];
	}

	let neighbors: Vec<((i32, i32), HashSet<(i32, i32)>)> = [
		(current.0 +1, current.1),
		(current.0 -1, current.1),
		(current.0, current.1 +1),
		(current.0, current.1 -1),
	].iter().filter_map(|&neighbor| {
		if neighbor.0 < 0 || neighbor.0 >= matrix.len() as i32 || neighbor.1 < 0 || neighbor.1 >= matrix.len() as i32 {
			return None;
		}
		
		if matrix[neighbor.0 as usize][neighbor.1 as usize] == '#' {
			return None;
		}

		if path.contains(&neighbor) {
			return None;
		}

		let mut neighbor = neighbor;
		let mut n_path = HashSet::new();
		n_path.insert(neighbor);

		while ['<', '>', '^', 'v'].contains(&matrix[neighbor.0 as usize][neighbor.1 as usize]) {
			neighbor = match matrix[neighbor.0 as usize][neighbor.1 as usize] {
				'>' => (neighbor.0, neighbor.1+1),
				'<' => (neighbor.0, neighbor.1-1),
				'^' => (neighbor.0-1, neighbor.1),
				'v' => (neighbor.0+1, neighbor.1),
				_ => unreachable!()
			};
			if path.contains(&neighbor) {
				return None;
			}
			n_path.insert(neighbor);
		}

		return Some((neighbor, n_path));
	}).collect();
	
	if neighbors.len() == 1 {
		for &x in neighbors[0].1.iter() {
			path.insert(x);
		}
		return longest_path_p1(matrix, path, neighbors[0].0);
	} 

	return neighbors.iter().flat_map(|neighbor| {
		let mut copy = path.clone();
		for &x in neighbor.1.iter() {
			copy.insert(x);
		}
		return longest_path_p1(matrix, copy, neighbor.0);
	}).collect();
}

fn longest_path_p2(matrix: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> u32 {
	let mut bindings = Vec::new();
	bindings.push(start);
	bindings.push(end);

	for x in 0..matrix.len() {
		for y in 0..matrix[0].len() {
			if matrix[x][y] == '#' {
				continue;
			}

			let (x,y) = (x as i32, y as i32);

			let neighbors = [
				(x +1, y),
				(x -1, y),
				(x, y +1),
				(x, y -1),
			].iter().filter(|&neighbor| {
				if neighbor.0 < 0 || neighbor.0 >= matrix.len() as i32 || neighbor.1 < 0 || neighbor.1 >= matrix.len() as i32 {
					return false;
				}
				
				if matrix[neighbor.0 as usize][neighbor.1 as usize] == '#' {
					return false;
				}
				return true;
			}).count();

			if neighbors > 2 {
				bindings.push((x,y));
			}
		}
	}

	let mut edges = HashMap::new();
	
	for &binding in bindings.iter() {
		let curr = binding;

		let mut hits = Vec::new();

		let mut queue = Vec::new();
		let mut initial_path = Vec::new();
		initial_path.push(curr);
		queue.push((curr, initial_path));

		while let Some((next, path)) = queue.pop() {
			let (x, y) = next;
			let neighbors: Vec<(i32, i32)> = [
				(x +1, y),
				(x -1, y),
				(x, y +1),
				(x, y -1),
			].iter().filter_map(|&neighbor| {
				if neighbor.0 < 0 || neighbor.0 >= matrix.len() as i32 || neighbor.1 < 0 || neighbor.1 >= matrix.len() as i32 {
					return None;
				}
				
				if matrix[neighbor.0 as usize][neighbor.1 as usize] == '#' {
					return None;
				}

				if path.contains(&neighbor) {
					return None;
				}

				return Some(neighbor);
			}).collect();

			for neigh in neighbors.into_iter() {
				if bindings.contains(&neigh) {
					hits.push((neigh, path.len()));
					continue;
				}

				let mut n_path = path.clone();
				n_path.push(neigh);

				queue.push((neigh, n_path));
			}
		}
		
		edges.insert(binding, hits);
	}
	
	let mut queue = Vec::new();

	let mut path = Vec::new();
	path.push(start);
	queue.push((start, 0, path));

	let mut max = 0;

	while let Some((next, dist, path)) = queue.pop() {
		let neighbors = edges.get(&next).unwrap();

		for &neigh in neighbors.iter() {
			if path.contains(&neigh.0) {
				continue;
			}

			let dist = dist + neigh.1;

			if neigh.0 == end {
				max = max.max(dist);
				continue;
			}

			let mut n_path = path.clone();
			n_path.push(neigh.0);

			queue.push((neigh.0, dist, n_path));
		}
	}

	return max as u32;
}