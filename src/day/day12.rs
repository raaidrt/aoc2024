use super::super::stage::Stage;
use rayon::prelude::*;
pub use std::error::Error;

fn parse(s: &str) -> Vec<Vec<char>> {
    s.lines()
        .into_iter()
        .map(|line| line.trim().chars().into_iter().collect())
        .collect()
}

fn present(grid: &Vec<Vec<char>>, i: i64, j: i64, c: char) -> bool {
    if !(0 <= i && i < grid.len() as i64 && 0 <= j && j < grid[0].len() as i64) {
        return false;
    }
    return grid[i as usize][j as usize] == c;
}

fn corners(grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let c = grid[i][j];
    (-1..=1)
        .into_par_iter()
        .flat_map(|di: i64| {
            (-1..=1).into_par_iter().filter_map(move |dj: i64| {
                if di.abs() == dj.abs() && di.abs() == 1 {
                    let (di1, dj1) = (di, 0);
                    let (di2, dj2) = (0, dj);
                    let i = i as i64;
                    let j = j as i64;
                    if !present(grid, i + di1, j + dj1, c) && !present(grid, i + di2, j + dj2, c) {
                        return Some(());
                    } else if present(grid, i + di1, j + dj1, c)
                        && present(grid, i + di2, j + dj2, c)
                        && !present(grid, i + di, j + dj, c)
                    {
                        Some(())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .count()
}

fn neighbors(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    (-1..=1)
        .into_par_iter()
        .flat_map(|di: i64| {
            (-1..=1).into_par_iter().filter_map(move |dj: i64| {
                if di.abs() == dj.abs() {
                    None
                } else {
                    let i: i64 = i as i64;
                    let j: i64 = j as i64;
                    if 0 <= i + di
                        && i + di < grid.len() as i64
                        && 0 <= j + dj
                        && j + dj < grid[0].len() as i64
                        && grid[i as usize][j as usize]
                            == grid[(i + di) as usize][(j + dj) as usize]
                    {
                        Some(((i + di) as usize, (j + dj) as usize))
                    } else {
                        None
                    }
                }
            })
        })
        .collect()
}

fn dfs(grid: &Vec<Vec<char>>, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> (usize, usize) {
    visited[i][j] = true;
    let nbors = neighbors(grid, i, j);
    let mut total_perim = 4 - nbors.len();
    let mut total_area = 1;
    for (i1, j1) in nbors {
        if !visited[i1][j1] {
            let (area, perimeter) = dfs(grid, i1, j1, visited);
            total_area += area;
            total_perim += perimeter;
        }
    }
    (total_area, total_perim)
}

fn dfs_corners(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    visited[i][j] = true;
    let nbors = neighbors(grid, i, j);
    let mut total_sides = corners(grid, i, j);
    let mut total_area = 1;
    for (i1, j1) in nbors {
        if !visited[i1][j1] {
            let (area, sides) = dfs_corners(grid, i1, j1, visited);
            total_area += area;
            total_sides += sides;
        }
    }
    (total_area, total_sides)
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let grid = parse(s);
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
            let (mut total_area, mut total_perim) = (vec![], vec![]);
            (0..grid.len()).into_iter().for_each(|i| {
                (0..grid[0].len()).into_iter().for_each(|j| {
                    if !visited[i][j] {
                        let (area, perimeter) = dfs(&grid, i, j, &mut visited);
                        total_area.push(area);
                        total_perim.push(perimeter);
                    }
                })
            });

            let result = total_area
                .par_iter()
                .zip(total_perim)
                .map(|(area, perim)| area * perim)
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let grid = parse(s);
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
            let (mut total_area, mut total_perim) = (vec![], vec![]);
            (0..grid.len()).into_iter().for_each(|i| {
                (0..grid[0].len()).into_iter().for_each(|j| {
                    if !visited[i][j] {
                        let (area, perimeter) = dfs_corners(&grid, i, j, &mut visited);
                        total_area.push(area);
                        total_perim.push(perimeter);
                    }
                })
            });

            let result = total_area
                .par_iter()
                .zip(total_perim)
                .map(|(area, perim)| area * perim)
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
