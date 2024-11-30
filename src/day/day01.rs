use super::super::stage::{Stage, StageUnimplemented};
use fraction::GenericFraction;
use rayon::prelude::*;
pub use std::error::Error;

const LOWER: i128 = 200000000000000;
const UPPER: i128 = 400000000000000;

#[derive(Debug, Clone)]
struct Position {
    x: i128,
    y: i128,
}

#[derive(Debug, Clone)]
struct Velocity {
    dx: i128,
    dy: i128,
}
fn f(u: i128) -> GenericFraction<u128> {
    GenericFraction::from(u)
}

fn in_bounds(l: i128, u: i128) -> Box<dyn Fn(GenericFraction<u128>) -> bool> {
    return Box::new(move |x: GenericFraction<u128>| f(l) <= x && x <= f(u));
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let lines: Vec<&str> = s.trim().lines().into_iter().collect();
            let vec_pos: Vec<(Position, Velocity)> = lines
                .par_iter()
                .map(|line| {
                    let pv: Vec<&str> = line.trim().split('@').collect();
                    let pos: Vec<i128> = pv[0]
                        .split(',')
                        .map(|i| i.trim().parse().expect("position"))
                        .collect();
                    let vec: Vec<i128> = pv[1]
                        .split(',')
                        .map(|i| i.trim().parse().expect("vector"))
                        .collect();
                    (
                        Position {
                            x: pos[0],
                            y: pos[1],
                        },
                        Velocity {
                            dx: vec[0],
                            dy: vec[1],
                        },
                    )
                })
                .collect();
            let combinations: Vec<Vec<((Position, Velocity), (Position, Velocity))>> = vec_pos
                .par_iter()
                .enumerate()
                .map(|(i, vp1)| {
                    vec_pos
                        .par_iter()
                        .enumerate()
                        .filter_map(|(j, vp2)| {
                            if i < j {
                                Some((vp1.clone(), vp2.clone()))
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .collect();
            let combinations: Vec<((Position, Velocity), (Position, Velocity))> =
                combinations.into_par_iter().flatten().collect();
            let result = combinations
                .par_iter()
                .map(|((p1, v1), (p2, v2))| {
                    // (x, y) = (x0, y0) + t * (dx, dy)
                    // y = dy / dx * x + (y0 - x0/dx * dy)
                    //
                    let m1 = f(v1.dy) / f(v1.dx);
                    let c1 = f(p1.y) - f(p1.x) / f(v1.dx) * f(v1.dy);

                    let m2 = f(v2.dy) / f(v2.dx);
                    let c2 = f(p2.y) - f(p2.x) / f(v2.dx) * f(v2.dy);

                    // y = m1 * x + c1, y = m2 * x + c2
                    // intersection when m1 * x + c1 = m2 * x + c2
                    // (m1 - m2) * x = c2 - c1
                    // x = (c2 - c1) / (m1 - m2)
                    // y = m1 * x + c1

                    let x = (c2 - c1) / (m1 - m2);
                    let y = m1 * x + c1;

                    let t1 = (x - f(p1.x)) / f(v1.dx);
                    let t2 = (x - f(p2.x)) / f(v2.dx);

                    if t1 < f(0) || t2 < f(0) {
                        0
                    } else if in_bounds(LOWER, UPPER)(x) && in_bounds(LOWER, UPPER)(y) {
                        1
                    } else {
                        0
                    }
                })
                .reduce(|| 0, |x: i128, y: i128| x + y);

            Ok(result.to_string())
        }
        Stage::B => Err(Box::new(StageUnimplemented(Stage::B))),
    }
}
