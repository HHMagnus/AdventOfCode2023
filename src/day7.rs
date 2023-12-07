use std::{fs, cmp::Ordering, collections::HashMap, iter::zip};

use itertools::Itertools;

#[derive(Debug)]
struct Hand {
	cards: Vec<char>,
	winning: u32
}

pub fn day7() {
	let file = fs::read_to_string("input/day7.txt").expect("Should have read file");

	let mut hands: Vec<Hand> = file.lines().map(|line| {
		let split: Vec<&str> = line.split(" ").collect();
		return Hand {
			cards: split[0].chars().collect(),
			winning: split[1].parse().unwrap()
		}
	}).collect();

	let map: HashMap<char, u32> = [
		('A', 20),
		('K', 19),
		('Q', 18),
		('J', 17),
		('T', 16),
		('9', 9),
		('8', 8),
		('7', 7),
		('6', 6),
		('5', 5),
		('4', 4),
		('3', 3),
		('2', 2)
	].iter().cloned().collect();

	hands.sort_by(|a, b| {
		let type1 = typ(a.cards.clone());
		let type2 = typ(b.cards.clone());
		if type1 == type2 {
			for (a,b) in zip(a.cards.iter(), b.cards.iter()) {
				let mapped1 = map.get(a).unwrap();
				let mapped2 = map.get(b).unwrap();
				if mapped1 != mapped2 {
					return mapped1.cmp(mapped2);
				}
			}
			return std::cmp::Ordering::Equal;
		} else if type1 < type2 {
			return std::cmp::Ordering::Less;
		} else {
			return std::cmp::Ordering::Greater;
		}
	});

	let part1: u32 = hands.iter().enumerate().map(|x| ((x.0+1) as u32) * x.1.winning).sum();

	println!("Day 7 part 1: {}", part1);

	let map2: HashMap<char, u32> = [
		('A', 20),
		('K', 19),
		('Q', 18),
		('T', 16),
		('9', 9),
		('8', 8),
		('7', 7),
		('6', 6),
		('5', 5),
		('4', 4),
		('3', 3),
		('2', 2),
		('J', 1),
	].iter().cloned().collect();

	let mut hands2: Vec<(u32, &Hand)> = hands.iter().map(|x| {
		let ranking = typ2(&x.cards);
		return (ranking, x)
	}).collect();
	hands2.sort_by(|a, b| {
		let type1 = a.0;
		let type2 = b.0;
		if type1 == type2 {
			for (a,b) in zip(a.1.cards.iter(), b.1.cards.iter()) {
				let mapped1 = map2.get(a).unwrap();
				let mapped2 = map2.get(b).unwrap();
				if mapped1 != mapped2 {
					return mapped1.cmp(mapped2);
				}
			}
			return std::cmp::Ordering::Equal;
		} else if type1 < type2 {
			return std::cmp::Ordering::Less;
		} else {
			return std::cmp::Ordering::Greater;
		}
	});
	let hands2: Vec<&Hand> = hands2.iter().map(|x| x.1).collect();

	let part2: u32 = hands2.iter().enumerate().map(|x| ((x.0+1) as u32) * x.1.winning).sum();

	println!("Day 8 part 2: {}", part2);
}

fn typ2(cards: &Vec<char>) -> u32 {
	return replace_j(&cards).iter().map(|x| typ(x.clone())).max().unwrap();
}

fn replace_j(cards: &Vec<char>) -> Vec<Vec<char>> {
	if !cards.contains(&'J') {
		return vec![cards.clone()];
	}
	
	let replacements = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

	let str: String = cards.iter().collect();

	let replaced: Vec<Vec<char>> = replacements.iter().map(|c| str.replacen('J', c.to_string().as_str(), 1).chars().collect()).collect();

	return replaced.iter().flat_map(replace_j).collect();
}

fn typ(cards: Vec<char>) -> u32{
	let unique:Vec<(char, u32)> = cards.iter().unique().map(|x| (*x, cards.iter().filter(|y| *y == x).count() as u32)).collect();
	let len = unique.len();
	if len == 1 {
		return 6;
	}
	if len == 2 {
		let order = unique.iter().map(|x| x.1).reduce(|a,b| a*b);
		return match order {
			Some(4) => 5,
			Some(6) => 4,
			Some(_) => panic!("wtf"),
			None => panic!("wtf")
		};
	}
	if len == 3 {
		if unique.iter().any(|x| x.1 == 3) {
			return 3;
		}
		return 2;
	}
	if len == 4 {
		return 1;
	}
	return 0;
}
