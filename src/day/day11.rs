use super::super::stage::{Stage, StageUnimplemented};
use rayon::prelude::*;
use std::collections::HashMap;
pub use std::error::Error;
use tqdm::tqdm;

fn parse(s: &str) -> Vec<usize> {
    s.trim().split(" ").map(|x| x.parse().unwrap()).collect()
}

fn num_digits(x: usize) -> usize {
    if x == 0 {
        0
    } else {
        1 + num_digits(x / 10)
    }
}

// x^y
fn pow(x: usize, y: usize) -> usize {
    if y == 0 {
        1
    } else {
        x * pow(x, y - 1)
    }
}

fn act(x: usize) -> Vec<usize> {
    let num_digits = num_digits(x);
    if x == 0 {
        return vec![1];
    }
    if num_digits % 2 == 0 {
        let pow = pow(10, num_digits / 2);
        let a = x / pow * pow;
        let b = x - a;
        let a = a / pow;
        vec![a, b]
    } else {
        vec![x * 2024]
    }
}

fn take_step(config: &Vec<usize>) -> Vec<usize> {
    config.par_iter().flat_map(|x| act(*x)).collect()
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let mut config = parse(s);
            for _ in 0..25 {
                config = take_step(&config);
            }
            let result = config.len();
            Ok(result.to_string())
        }
        Stage::B => {
            let config = parse(s);
            let mut freqs: HashMap<usize, usize> = HashMap::new();
            for c in config.iter() {
                match freqs.get(&c) {
                    Some(x) => {
                        freqs.insert(*c, x + 1);
                    }
                    None => {
                        freqs.insert(*c, 1);
                    }
                }
            }
            for _ in 0..75 {
                let mut new_freqs: HashMap<usize, usize> = HashMap::new();
                for (k, v) in freqs.iter() {
                    let xs = take_step(&vec![*k]);
                    for x in xs {
                        match new_freqs.get(&x) {
                            Some(ff) => {
                                new_freqs.insert(x, *ff + *v);
                            }
                            None => {
                                new_freqs.insert(x, *v);
                            }
                        }
                    }
                }
                freqs = new_freqs;
            }
            let result = freqs.par_iter().map(|(_, v)| *v).reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
