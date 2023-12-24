use std::fs;

use geo::{coord, Line, line_intersection::line_intersection};

use z3::ast::Ast;

pub fn day24() {
	let file = fs::read_to_string("input/day24.txt").expect("Should have read file");

	let points: Vec<((i128, i128, i128), (i128, i128, i128))> = file.lines().map(|x| {
		let split1: Vec<&str> = x.split(" @ ").collect();
		let split2: Vec<&str> = split1[0].split(", ").collect();
		let split3: Vec<&str> = split1[1].split(", ").collect();

		((split2[0].parse().unwrap(), split2[1].parse().unwrap(), split2[2].parse().unwrap()), (split3[0].parse().unwrap(), split3[1].parse().unwrap(), split3[2].parse().unwrap()))
	}).collect();
	
	let test_min: i128 = 200000000000000;
	let test_max: i128 = 400000000000000;

	let mut part1 = 0;

	for i in 0..points.len() {
		for j in i+1..points.len() {
			let p1 = points[i];
			let p2 = points[j];

			let line1 = Line::new(coord! {x: p1.0.0 as f64, y: p1.0.1 as f64}, coord! {x: p1.0.0 as f64 + p1.1.0 as f64 * test_max as f64, y: p1.0.1 as f64 + p1.1.1 as f64 * test_max as f64});
			let line2 = Line::new(coord! {x: p2.0.0 as f64, y: p2.0.1 as f64}, coord! {x: p2.0.0 as f64 + p2.1.0 as f64 * test_max as f64, y: p2.0.1 as f64 + p2.1.1 as f64 * test_max as f64});

			match line_intersection(line1, line2) {
				None => (),
				Some(x) => match x {
					geo::LineIntersection::SinglePoint { intersection, is_proper: _ } => {
						if intersection.x > test_min as f64 && intersection.x < test_max as f64 && intersection.y > test_min as f64 && intersection.y < test_max as f64 {
							part1 += 1;
						}
					},
					_ => ()
				}
			}
		}
	}

	println!("Day 24 part 1: {}", part1);

	let part2 = solve_part2(&points);

	println!("Day 24 part 2: {}", part2);
}

fn solve_part2(points: &Vec<((i128, i128, i128), (i128, i128, i128))>) -> i128 {
	let config = z3::Config::new();
	let context = z3::Context::new(&config);
	let solver = z3::Solver::new(&context);

	let x = z3::ast::Int::new_const(&context, "x");
    let y = z3::ast::Int::new_const(&context, "y");
    let z = z3::ast::Int::new_const(&context, "z");
    let vx = z3::ast::Int::new_const(&context, "vx");
    let vy = z3::ast::Int::new_const(&context, "vy");
    let vz = z3::ast::Int::new_const(&context, "vz");

    for (i, point) in points.iter().enumerate() {
		let point = ((point.0.0 as i64, point.0.1 as i64, point.0.2 as i64), (point.1.0 as i64, point.1.1 as i64, point.1.2 as i64));
        let a = z3::ast::Int::from_i64(&context, point.0.0);
        let va = z3::ast::Int::from_i64(&context, point.1.0);
        let b = z3::ast::Int::from_i64(&context, point.0.1);
        let vb = z3::ast::Int::from_i64(&context, point.1.1);
        let c = z3::ast::Int::from_i64(&context, point.0.2);
        let vc = z3::ast::Int::from_i64(&context, point.1.2);

        let t = z3::ast::Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }

	if solver.check() == z3::SatResult::Sat {
        let Some(m) = solver.get_model() else {
            println!("Failed to solve!");
            return 0;
        };
        return m.eval(&(x + y + z), true).unwrap().as_i64().unwrap() as i128;
    }

	return 0;
}