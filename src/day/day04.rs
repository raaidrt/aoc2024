use super::super::stage::Stage;
use rayon::prelude::*;
pub use std::error::Error;
use std::sync::Arc;

fn parse(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .into_iter()
        .map(|s| s.trim().as_bytes().to_vec())
        .collect()
}

fn is_xmas(grid: Arc<Vec<Vec<u8>>>, i: usize, j: usize, di: i32, dj: i32, bytes: Vec<u8>) -> i32 {
    let i = i as i32;
    let j = j as i32;
    let len = bytes.len() as i32;
    if i + (len - 1) * di >= grid.len() as i32 {
        return 0;
    }
    if i + (len - 1) * di < 0 {
        return 0;
    }
    if j + (len - 1) * dj >= grid[0].len() as i32 {
        return 0;
    }
    if j + (len - 1) * dj < 0 {
        return 0;
    }

    for k in 0..len {
        // println!("checking grid_{row}_{col} ({g}) ?= xmas_{k} = {x}");
        if grid[(i + k * di) as usize][(j + k * dj) as usize] != bytes[k as usize] {
            return 0;
        }
    }
    return 1;
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let grid = Arc::new(parse(s));
            let result = (0..grid.len())
                .into_par_iter()
                .flat_map(move |i| {
                    let grid = grid.clone();
                    (0..grid[i].len()).into_par_iter().flat_map(move |j| {
                        let grid = grid.clone();
                        [-1, 0, 1].into_par_iter().flat_map(move |di| {
                            let grid = grid.clone();
                            [-1, 0, 1].into_par_iter().map(move |dj| {
                                let grid = grid.clone();
                                if di == dj && di == 0 {
                                    0
                                } else {
                                    is_xmas(grid, i, j, di, dj, "XMAS".as_bytes().to_vec())
                                }
                            })
                        })
                    })
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let grid = Arc::new(parse(s));
            let result = (0..grid.len())
                .into_par_iter()
                .flat_map(move |i| {
                    let grid = grid.clone();
                    (0..grid[i].len()).into_par_iter().flat_map(move |j| {
                        let grid = grid.clone();
                        [-1, 1].into_par_iter().flat_map(move |dd1| {
                            let grid = grid.clone();
                            [-1, 1].into_par_iter().map(move |dd2| {
                                is_xmas(
                                    grid.clone(),
                                    i,
                                    j,
                                    dd1,
                                    dd1,
                                    "MA".as_bytes().to_vec().iter().rev().map(|x| *x).collect(),
                                ) * is_xmas(
                                    grid.clone(),
                                    i,
                                    j,
                                    -dd1,
                                    -dd1,
                                    "AS".as_bytes().to_vec(),
                                ) * is_xmas(
                                    grid.clone(),
                                    i,
                                    j,
                                    dd2,
                                    -dd2,
                                    "MA".as_bytes().to_vec().iter().rev().map(|x| *x).collect(),
                                ) * is_xmas(grid.clone(), i, j, -dd2, dd2, "AS".as_bytes().to_vec())
                            })
                        })
                    })
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
