use std::{fs, collections::HashMap};

#[derive(Debug)]
enum Cmp {
	Greater,
	Less
}

#[derive(Debug)]
enum Rule {
	Else (String),
	If (String, Cmp, u32, String)
}

#[derive(Debug)]
struct Part {
	x: u32,
	m: u32,
	a: u32,
	s: u32
}

pub fn day19() {
	let file = fs::read_to_string("input/day19.txt").expect("Should have read file");

	let s: Vec<&str> = file.split("\n\n").collect();

	let workflows: HashMap<&str, Vec<Rule>> = s[0].lines().map(|workflow| {
		let s1: Vec<&str> = workflow.split("{").collect();
		let name = s1[0];
		let s2 = s1[1].strip_suffix(|_| true).unwrap();
		let rules = s2.split(",").map(|rule| {
			if rule.contains("<") {
				let s: Vec<&str> = rule.split("<").collect();
				let n = s[0].to_string();
				let s: Vec<&str> = s[1].split(":").collect();
				Rule::If(n, Cmp::Less, s[0].parse().unwrap(), s[1].to_string())
			} else if rule.contains(">") {
				let s: Vec<&str> = rule.split(">").collect();
				let n = s[0].to_string();
				let s: Vec<&str> = s[1].split(":").collect();
				Rule::If(n, Cmp::Greater, s[0].parse().unwrap(), s[1].to_string())
			} else {
				Rule::Else(rule.to_string())
			}
		}).collect();
		(name, rules)
	}).collect();
	let parts: Vec<Part> = s[1].lines().map(|part| {
		let s: Vec<&str> = part.strip_prefix(|_| true).unwrap().strip_suffix(|_| true).unwrap().split(",").collect();
		let x = s[0].split("=").last().unwrap().parse().unwrap();
		let m = s[1].split("=").last().unwrap().parse().unwrap();
		let a = s[2].split("=").last().unwrap().parse().unwrap();
		let s = s[3].split("=").last().unwrap().parse().unwrap();
		Part {
			x: x,
			m: m,
			a: a,
			s: s
		}
	}).collect();

	let mut part1 = 0;

	for part in parts {
		let mut curr = "in";
		while curr != "R" && curr != "A" {
			let wf = workflows.get(curr).unwrap();
			for r in wf {
				match r {
					Rule::If(name, cmp, val, dest) => {
						let cmp_against = match name.as_str() {
							"a" => part.a,
							"x" => part.x,
							"m" => part.m,
							"s" => part.s,
							_ => panic!("Unknown name {}", name)
						};
						match cmp {
							Cmp::Greater => {
								if cmp_against > *val {
									curr = dest;
									break;
								}
							},
							Cmp::Less => {
								if cmp_against < *val {
									curr = dest;
									break;
								}
							}
						}
					},
					Rule::Else(name) => {
						curr = name;
						break;
					}
				}
			}
		}

		if curr == "A" {
			part1 += part.x + part.m + part.a + part.s;
		}
	}

	println!("Day 19 part 1: {}", part1);

	let part2 = solve_part2(&workflows, (1, 4000), (1, 4000), (1, 4000), (1, 4000), "in");

	println!("Day 19 part 2: {}", part2);
}

fn solve_part2(workflows: &HashMap<&str, Vec<Rule>>, mut x: (u32, u32), mut m: (u32, u32), mut a: (u32, u32), mut s: (u32, u32), curr: &str) -> u64 {
	if curr == "A" {
		let xsd = (x.1 as u64 - x.0 as u64 + 1) * (m.1 as u64 - m.0 as u64 + 1) * (a.1 as u64 - a.0 as u64 + 1) * (s.1 as u64 - s.0 as u64 + 1);
		return xsd;
	}

	if curr == "R" {
		return 0;
	}

	let workflow = workflows.get(curr).unwrap();
	let mut res = 0;
	for rule in workflow {
		match rule {
			Rule::Else(name) => {
				res += solve_part2(workflows, x, m, a, s, name);
			},
			Rule::If(name, cmp, val, dest) => {
				match cmp {
					Cmp::Greater => {
						match name.as_str() {
							"x" => {
								if let Some(v1) = greater_then(x, *val) {
									if let Some(v2) = v1.1 {
										let off_x = v1.0;
										x = v2;
										res += solve_part2(workflows, off_x, m, a, s, dest);
									} else {
										return solve_part2(workflows, x, m, a, s, dest)
									}
								}
							},
							"m" => {
								if let Some(v1) = greater_then(m, *val) {
									if let Some(v2) = v1.1 {
										let off_m = v1.0;
										m = v2;
										res += solve_part2(workflows, x, off_m, a, s, dest);
									} else {
										return solve_part2(workflows, x, m, a, s, dest)
									}
								}
							},
							"a" => {
								if let Some(v1) = greater_then(a, *val) {
									if let Some(v2) = v1.1 {
										let off_a = v1.0;
										a = v2;
										res += solve_part2(workflows, x, m, off_a, s, dest);
									} else {
										return solve_part2(workflows, x, m, a, s, dest)
									}
								}
							}
							"s" => {
								if let Some(v1) = greater_then(s, *val) {
									if let Some(v2) = v1.1 {
										let off_s = v1.0;
										s = v2;
										res += solve_part2(workflows, x, m, a, off_s, dest);
									} else {
										return solve_part2(workflows, x, m, a, s, dest)
									}
								}
							},
							_ => panic!("unknown {}", name)
						}
					},
					Cmp::Less => {
						match name.as_str() {
							"x" => {
								if let Some(v1) = less_then(x, *val) {
									if let Some(v2) = v1.1 {
										let off_x = v1.0;
										x = v2;
										res += solve_part2(workflows, off_x, m, a, s, dest)
									} else {
										return solve_part2(workflows, x, m, a, s, dest)
									}
								}
							},
							"m" => {
								if let Some(v1) = less_then(m, *val) {
									if let Some(v2) = v1.1 {
										let off_m = v1.0;
										m = v2;
										res += solve_part2(workflows, x, off_m, a, s, dest)
									} else {
										return solve_part2(workflows, x, m, a, s, dest)
									}
								}
							},
							"a" => {
								if let Some(v1) = less_then(a, *val) {
									if let Some(v2) = v1.1 {
										let off_a = v1.0;
										a = v2;
										res += solve_part2(workflows, x, m, off_a, s, dest)
									} else {
										return solve_part2(workflows, x, m, a, s, dest)
									}
								}
							},
							"s" => {
								if let Some(v1) = less_then(s, *val) {
									if let Some(v2) = v1.1 {
										let off_s = v1.0;
										s = v2;
										res += solve_part2(workflows, x, m, a, off_s, dest)
									} else {
										return solve_part2(workflows, x, m, a, s, dest)
									}
								}
							},
							_ => panic!("unknown {}", name)
						}
					}
				}
			} 
		}
	}
	return res;
}

fn less_then(x: (u32, u32), val: u32) -> Option<((u32, u32), Option<(u32, u32)>)> {
	if x.1 < val {
		return Some((x, Option::None))
	}

	if x.0 < val {
		return Some(((x.0, val-1), Some((val, x.1))))
	}

	return None;
}

fn greater_then(x: (u32, u32), val: u32) -> Option<((u32, u32), Option<(u32, u32)>)> {
	if x.0 > val {
		return Some((x, Option::None))
	}

	if x.1 > val {
		return Some(((val+1, x.1), Some((x.0, val))))
	}

	return None;
}