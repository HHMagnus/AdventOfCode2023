use std::fs;

#[derive(Clone)]
struct Pair {
	color: String,
	num: u32
}

#[derive(Clone)]
struct Hand {
	cubes: Vec<Pair>
}

#[derive(Clone)]
struct Game {
	id: u32,
	hands: Vec<Hand>
}

pub fn day2() {
	let file = fs::read_to_string("input/day2.txt").expect("Should have read file");

	let mut games = Vec::new();

	for line in file.lines() {
		let l: Vec<&str> = line.split(": ").collect();
		let id:u32 = l[0][5..l[0].len()].parse().unwrap();
		let mut hands = Vec::new();
		let s: Vec<&str> = l[1].split("; ").collect();
		for x in s {
			let mut cubs = Vec::new();
			let c: Vec<&str> = x.split(", ").collect();
			for p in c {
				let f: Vec<&str> = p.split(" ").collect();
				let num = f[0].parse().unwrap();
				let color = f[1];
				cubs.push(Pair {
					num: num,
					color: color.to_string()
				});
			}
			hands.push(Hand {
				cubes: cubs
			});
		}
		let game = Game {
			id: id,
			hands: hands
		};
		games.push(game);
	}

	let mut ids = 0;
	for game in games.clone() {
		let mut valid = true;
		for hand in game.hands {
			for pair in hand.cubes {
				let max_num = match pair.color.as_str() {
					"red" => 12,
					"green" => 13,
					"blue" => 14,
					_ => panic!("{}", pair.color)
				};

				if max_num < pair.num {
					valid = false;
				}
			}
		}
		
		if valid {
			ids += game.id;
		}
	}

	println!("Day 1 part 1: {}", ids);

	let mut ps_sum = 0;
	for game in games {
		let mut red = 0;
		let mut green = 0;
		let mut blue = 0;

		for hand in game.hands {
			for pair in hand.cubes {
				match pair.color.as_str() {
					"red" => if red < pair.num { red = pair.num },
					"green" => if green < pair.num { green = pair.num },
					"blue" => if blue < pair.num { blue = pair.num },
					_ => panic!("{}", pair.color)
				}
			}
		}

		ps_sum += red * green * blue;
	}

	println!("Day 1 part 2: {}", ps_sum);
}