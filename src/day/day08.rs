use super::super::stage::Stage;
use dashmap::{DashMap, DashSet};
use rayon::prelude::*;
pub use std::error::Error;

fn parse(s: &str) -> (DashMap<char, Vec<(usize, usize)>>, Vec<Vec<Option<char>>>) {
    let map: DashMap<char, Vec<(usize, usize)>> = DashMap::new();
    ('0'..='9').into_iter().for_each(|c| {
        map.insert(c, vec![]);
    });
    ('a'..='z').into_iter().for_each(|c| {
        map.insert(c, vec![]);
    });
    ('A'..='Z').into_iter().for_each(|c| {
        map.insert(c, vec![]);
    });
    s.lines().into_iter().enumerate().for_each(|(i, line)| {
        line.trim()
            .chars()
            .into_iter()
            .enumerate()
            .for_each(|(j, c)| {
                if c.is_alphanumeric() {
                    let v = map.get_mut(&c);
                    (*(v.unwrap())).push((i, j));
                }
            });
    });
    let grid = s
        .lines()
        .into_iter()
        .map(|line| {
            line.trim()
                .chars()
                .into_iter()
                .map(|c| if c.is_alphanumeric() { Some(c) } else { None })
                .collect()
        })
        .collect();
    (map, grid)
}

fn in_bounds(nr: i64, nc: i64, r: i64, c: i64) -> bool {
    0 <= r && r < nr && 0 <= c && c < nc
}

fn antinodes_for(
    nr: usize,
    nc: usize,
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
) -> Vec<(usize, usize)> {
    let (nr, nc, r1, r2, c1, c2) = (
        nr as i64, nc as i64, r1 as i64, r2 as i64, c1 as i64, c2 as i64,
    );
    let dr: i64 = r2 - r1;
    let dc: i64 = c2 - c1;
    let (anr1, anc1) = (r1 - dr, c1 - dc);
    let (anr2, anc2) = (r2 + dr, c2 + dc);
    let mut result = vec![];
    if in_bounds(nr, nc, anr1, anc1) {
        result.push((anr1 as usize, anc1 as usize));
    }
    if in_bounds(nr, nc, anr2, anc2) {
        result.push((anr2 as usize, anc2 as usize));
    }
    result
}

fn antinodes_for2(
    nr: usize,
    nc: usize,
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
) -> Vec<(usize, usize)> {
    let (nr, nc, r1, r2, c1, c2) = (
        nr as i64, nc as i64, r1 as i64, r2 as i64, c1 as i64, c2 as i64,
    );
    let dr: i64 = r2 - r1;
    let dc: i64 = c2 - c1;
    let (mut anr1, mut anc1) = (r1 - dr, c1 - dc);
    let (mut anr2, mut anc2) = (r2 + dr, c2 + dc);
    let mut result = vec![];
    while in_bounds(nr, nc, anr1, anc1) {
        result.push((anr1 as usize, anc1 as usize));
        anr1 -= dr;
        anc1 -= dc;
    }
    while in_bounds(nr, nc, anr2, anc2) {
        result.push((anr2 as usize, anc2 as usize));
        anr2 += dr;
        anc2 += dc;
    }
    result.push((r1 as usize, c1 as usize));
    result.push((r2 as usize, c2 as usize));
    result
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let (map, grid) = parse(s);
            let nr = grid.len();
            let nc = grid[0].len();
            let antinodes: DashSet<(usize, usize)> = DashSet::new();
            map.iter().for_each(|x| {
                x.par_iter().enumerate().for_each(|(i, (r1, c1))| {
                    x.par_iter().enumerate().for_each(|(j, (r2, c2))| {
                        if i < j {
                            let an = antinodes_for(nr, nc, *r1, *c1, *r2, *c2);
                            an.par_iter().for_each(|coord| {
                                antinodes.insert(*coord);
                            });
                        }
                    });
                });
            });
            let result = antinodes.len();
            Ok(result.to_string())
        }
        Stage::B => {
            let (map, grid) = parse(s);
            let nr = grid.len();
            let nc = grid[0].len();
            let antinodes: DashSet<(usize, usize)> = DashSet::new();
            map.iter().for_each(|x| {
                x.par_iter().enumerate().for_each(|(i, (r1, c1))| {
                    x.par_iter().enumerate().for_each(|(j, (r2, c2))| {
                        if i < j {
                            let an = antinodes_for2(nr, nc, *r1, *c1, *r2, *c2);
                            an.par_iter().for_each(|coord| {
                                antinodes.insert(*coord);
                            });
                        }
                    });
                });
            });
            let result = antinodes.len();
            Ok(result.to_string())
        }
    }
}
