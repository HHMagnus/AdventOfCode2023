use std::{fs, collections::HashMap};

pub fn day22() {
	let file = fs::read_to_string("input/day22.txt").expect("Should have read file"); 

	let mut bricks: Vec<((u32, u32, u32), (u32, u32, u32))> = file.lines().map(|x| {
		let split: Vec<_> = x.split("~").collect();
		let p1: Vec<u32> = split[0].split(",").map(|x| x.parse().unwrap()).collect();
		let p2: Vec<u32> = split[1].split(",").map(|x| x.parse().unwrap()).collect();
		((p1[0], p1[1], p1[2]), (p2[0], p2[1], p2[2]))
	}).collect();

	bricks.sort_by(|x, y| x.0.2.min(x.1.2).cmp(&y.0.2.min(y.1.2)));

	let mut fallen_bricks: Vec<((u32, u32, u32), (u32, u32, u32))> = Vec::new();

	let mut blocks = HashMap::new();

	let mut saved_obstacles = HashMap::new();

	let mut laying_on = HashMap::new();

	let len: usize = bricks.len();

	for mut brick in bricks.into_iter() {
		while brick.0.2.min(brick.1.2) > 1 {
			let ours: Vec<(u32, u32, u32)> = grid(&((brick.0.0, brick.0.1, brick.0.2-1), (brick.1.0, brick.1.1, brick.1.2-1)));
			let obstacle = fallen_bricks.iter().filter(|&other| {
				let theirs: &Vec<(u32, u32, u32)> = saved_obstacles.get(other).unwrap();
				ours.iter().any(|x| theirs.contains(x))
			}).map(|x| x.clone()).collect::<Vec<_>>();
			if obstacle.len() > 0 {
				if obstacle.len() == 1 {
					let below = obstacle[0];
					blocks.insert(below, match blocks.get(&below) { Some(x) => x + 1, None => 1 });
				}
				laying_on.insert(brick, obstacle);
				break;
			} 
			
			brick.0.2 -= 1;
			brick.1.2 -= 1;
		}

		let ours: Vec<(u32, u32, u32)> = grid(&brick);

		saved_obstacles.insert(brick, ours);

		fallen_bricks.push(brick);
	}

	let part1 = fallen_bricks.len() - blocks.len();

	println!("Day 22 part 1: {}", part1);

	let mut part2 = 0;

	for brick in fallen_bricks {
		let mut rem = laying_on.clone();
		rem.remove(&brick);

		for x in rem.clone().iter().filter(|x| x.1.contains(&brick)) {
			let mut list = rem.get_mut(x.0).unwrap();
			let index = list.iter().position(|x| *x == brick).unwrap();
			list.remove(index);
		}

		let mut unblocks = 0;

		while let Some(unblocked) = rem.clone().iter().find(|x| x.1.is_empty()) {
			rem.remove(unblocked.0);

			for x in rem.clone().iter().filter(|x| x.1.contains(unblocked.0)) {
				let mut list = rem.get_mut(x.0).unwrap();
				let index = list.iter().position(|x| x == unblocked.0).unwrap();
				list.remove(index);
			}

			unblocks += 1;
		}

		part2 += unblocks;
	}

	println!("Day 22 part 2: {}", part2);
}

fn grid(brick: &((u32, u32, u32), (u32, u32, u32))) -> Vec<(u32, u32, u32)> {
	(brick.0.0..brick.1.0+1)
		.collect::<Vec<u32>>()
		.into_iter()
		.flat_map(|x|
			(brick.0.1..brick.1.1+1)
				.collect::<Vec<u32>>()
				.into_iter()
				.flat_map(move |y|
					(brick.0.2..brick.1.2+1)
						.collect::<Vec<u32>>()
						.into_iter()
						.map(|z| (x, y, z))
						.collect::<Vec<_>>()
				)
		).collect()
}