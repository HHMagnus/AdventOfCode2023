use std::{fs, collections::{BinaryHeap, HashSet, HashMap}, cmp::Ordering};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
	East,
	West,
	North,
	South
}

#[derive(Clone, Eq, PartialEq)]
struct Point {
	cost: u32,
	point: (usize, usize),
	direction: Direction,
	straight: u32,
}

impl Ord for Point {
	fn cmp(&self, other: &Self) -> Ordering {
		other.cost.cmp(&self.cost)
			.then_with(|| self.point.0.cmp(&other.point.0))
			.then_with(|| self.point.1.cmp(&other.point.1))
	}
}

impl PartialOrd for Point {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day17() {
	let file = fs::read_to_string("input/day17.txt").expect("Should have read file");

	let matrix: Vec<Vec<u32>> = file.lines().map(|x| x.chars().map(|x| x.to_string().parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect();

	part1(matrix.clone());
	part2(matrix);
}

fn part1(matrix: Vec<Vec<u32>>) {
	let mut visited = HashMap::new();

	let mut queue: BinaryHeap<Point> = BinaryHeap::new();

	queue.push(Point {
		cost: matrix[0][1],
		point: (0, 1),
		direction: Direction::East,
		straight: 1,
	});
	queue.push(Point {
		cost: matrix[1][0],
		point: (1, 0),
		direction: Direction::South,
		straight: 1,
	});

	while let Some(Point { cost, point, direction, straight }) = queue.pop() {
		let (x, y) = point;
		if x == matrix.len()-1 && y == matrix[0].len()-1 {
			println!("Day 17 part 1: {}", cost);
			break;
		}

		visited.insert((point, direction, straight), cost);

		let right = match direction {
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North,
			Direction::North => Direction::East,
		};

		let left = match direction {
			Direction::East => Direction::North,
			Direction::South => Direction::East,
			Direction::West => Direction::South,
			Direction::North => Direction::West,
		};

		let mut options = [(right, 1), (left, 1)].to_vec();

		if straight < 3 {
			options.push((direction, straight+1));
		}

		let neighbors: Vec<Point> = options.into_iter().map(|(d, s)| {
			(d, s, match d {
				Direction::East => (x as i32, y as i32 +1),
				Direction::North => (x as i32 -1, y as i32),
				Direction::South => (x as i32 +1, y as i32),
				Direction::West => (x as i32, y as i32 -1),
			})
		}).filter(|(_, _, (x, y))| *x >= 0 && *y >= 0 && *x < matrix.len() as i32 && *y < matrix[0].len() as i32)
			.map(|(d, s, (x, y))| Point {
				cost: cost + matrix[x as usize][y as usize],
				point: (x as usize, y as usize),
				direction: d,
				straight: s
			})
			.filter(|x| !visited.contains_key(&(x.point, x.direction, x.straight)) || visited.get(&(x.point, x.direction, x.straight)).unwrap() > &x.cost)
			.collect();

		for neighbor in neighbors {
			visited.insert((neighbor.point, neighbor.direction, neighbor.straight), neighbor.cost);
			queue.push(neighbor);
		}
	}
}

fn part2(matrix: Vec<Vec<u32>>) {
	let mut visited = HashMap::new();

	let mut queue: BinaryHeap<Point> = BinaryHeap::new();

	queue.push(Point {
		cost: matrix[0][1],
		point: (0, 1),
		direction: Direction::East,
		straight: 1,
	});
	queue.push(Point {
		cost: matrix[1][0],
		point: (1, 0),
		direction: Direction::South,
		straight: 1,
	});

	while let Some(Point { cost, point, direction, straight }) = queue.pop() {
		let (x, y) = point;
		if x == matrix.len()-1 && y == matrix[0].len()-1 {
			println!("Day 17 part 2: {}", cost);
			break;
		}

		visited.insert((point, direction, straight), cost);

		let mut options = Vec::new();

		if straight > 3 {
			let right = match direction {
				Direction::East => Direction::South,
				Direction::South => Direction::West,
				Direction::West => Direction::North,
				Direction::North => Direction::East,
			};
			options.push((right, 1));
	
			let left = match direction {
				Direction::East => Direction::North,
				Direction::South => Direction::East,
				Direction::West => Direction::South,
				Direction::North => Direction::West,
			};
			options.push((left, 1));
		}

		if straight < 10 {
			options.push((direction, straight+1));
		}

		let neighbors: Vec<Point> = options.into_iter().map(|(d, s)| {
			(d, s, match d {
				Direction::East => (x as i32, y as i32 +1),
				Direction::North => (x as i32 -1, y as i32),
				Direction::South => (x as i32 +1, y as i32),
				Direction::West => (x as i32, y as i32 -1),
			})
		}).filter(|(_, _, (x, y))| *x >= 0 && *y >= 0 && *x < matrix.len() as i32 && *y < matrix[0].len() as i32)
			.map(|(d, s, (x, y))| Point {
				cost: cost + matrix[x as usize][y as usize],
				point: (x as usize, y as usize),
				direction: d,
				straight: s
			})
			.filter(|x| !visited.contains_key(&(x.point, x.direction, x.straight)) || visited.get(&(x.point, x.direction, x.straight)).unwrap() > &x.cost)
			.collect();

		for neighbor in neighbors {
			visited.insert((neighbor.point, neighbor.direction, neighbor.straight), neighbor.cost);
			queue.push(neighbor);
		}
	}
}