use super::super::stage::Stage;
use dashmap::DashMap;
use rayon::prelude::*;
pub use std::error::Error;

fn parse(s: &str) -> (Vec<i32>, Vec<i32>) {
    let pairs: Vec<&str> = s.lines().into_iter().map(|s| s.trim()).collect();
    let (left, right): (Vec<i32>, Vec<i32>) = pairs
        .par_iter()
        .map(|line| {
            let pair: Vec<&str> = line.split("   ").collect();
            let (left, right): (i32, i32) = (pair[0].parse().unwrap(), pair[1].parse().unwrap());
            (left, right)
        })
        .unzip();
    (left, right)
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let (mut left, mut right) = parse(s);
            left.par_sort();
            right.par_sort();
            let result = left
                .par_iter()
                .zip(right.par_iter())
                .map(|(left, right)| (left - right).abs())
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let (left, right) = parse(s);
            let map = DashMap::new();
            right.par_iter().for_each(|x| {
                map.insert(*x, 0);
            });
            right.par_iter().for_each(|x| {
                map.alter(x, |_: &i32, v: i32| v + 1);
            });
            let result = left
                .par_iter()
                .map(|elem| map.get(elem).map_or_else(|| 0, |p| *p) * elem)
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
