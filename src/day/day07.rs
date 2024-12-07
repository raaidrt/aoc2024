use super::super::stage::{Stage, StageUnimplemented};
use rayon::prelude::*;
pub use std::error::Error;
use std::sync::{atomic::AtomicBool, Arc};

fn parse(s: &str) -> Vec<(u128, Vec<u128>)> {
    s.lines()
        .into_iter()
        .map(|line| {
            let result: Vec<&str> = line.trim().split(": ").collect();
            let lhs: u128 = result[0].trim().parse().unwrap();
            let rhs: Vec<u128> = result[1]
                .split(" ")
                .map(|x| x.trim().parse().unwrap())
                .collect();
            (lhs, rhs)
        })
        .collect()
}

fn solve(lhs: u128, rhs_left: u128, rhs_right: &[u128]) -> bool {
    if rhs_right.len() == 0 {
        lhs == rhs_left
    } else {
        let (first, second) = rayon::join(
            || solve(lhs, rhs_left + rhs_right[0], &rhs_right[1..]),
            || solve(lhs, rhs_left * rhs_right[0], &rhs_right[1..]),
        );
        first || second
    }
}

fn num_digits(x: u128) -> u128 {
    if x == 0 {
        0
    } else {
        1 + num_digits(x / 10)
    }
}

// x^2
fn pow(x: u128, y: u128) -> u128 {
    if y == 0 {
        1
    } else {
        x * pow(x, y - 1)
    }
}

fn concat(x: u128, y: u128) -> u128 {
    x * pow(10, num_digits(y)) + y
}

fn solve2(lhs: u128, rhs_left: u128, rhs_right: &[u128]) -> bool {
    if rhs_right.len() == 0 {
        lhs == rhs_left
    } else {
        let solved = AtomicBool::new(false);
        rayon::scope(|s| {
            s.spawn(|_| {
                if solve2(lhs, rhs_left + rhs_right[0], &rhs_right[1..]) {
                    solved.store(true, std::sync::atomic::Ordering::Relaxed);
                }
            });
            s.spawn(|_| {
                if solve2(lhs, rhs_left * rhs_right[0], &rhs_right[1..]) {
                    solved.store(true, std::sync::atomic::Ordering::Relaxed);
                }
            });
            s.spawn(|_| {
                if solve2(lhs, concat(rhs_left, rhs_right[0]), &rhs_right[1..]) {
                    solved.store(true, std::sync::atomic::Ordering::Relaxed);
                }
            });
        });
        solved.load(std::sync::atomic::Ordering::Relaxed)
    }
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let eqns = parse(s);
            let result = eqns
                .par_iter()
                .map(|(lhs, rhs)| {
                    (if rhs.len() == 0 {
                        if *lhs == 0 {
                            1
                        } else {
                            0
                        }
                    } else if solve(*lhs, rhs[0], &rhs[1..]) {
                        1
                    } else {
                        0
                    }) * (*lhs)
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let eqns = parse(s);
            let result = eqns
                .par_iter()
                .map(|(lhs, rhs)| {
                    (if rhs.len() == 0 {
                        if *lhs == 0 {
                            1
                        } else {
                            0
                        }
                    } else if solve2(*lhs, rhs[0], &rhs[1..]) {
                        1
                    } else {
                        0
                    }) * (*lhs)
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
