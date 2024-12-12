use super::super::stage::Stage;
use crate::graph::{AdjacencyListGraph, Graph};
use rayon::prelude::*;
use std::collections::HashMap;
pub use std::error::Error;

fn neighbors(i: usize, j: usize, grid: &Vec<Vec<Option<usize>>>) -> Vec<(usize, usize)> {
    if let Some(curr) = grid[i][j] {
        (-1..=1)
            .into_par_iter()
            .flat_map(|di: i64| {
                (-1..=1).into_par_iter().filter_map(move |dj: i64| {
                    if di.abs() == dj.abs() {
                        None
                    } else if !(0 <= (i as i64) + di
                        && (i as i64) + di < grid.len() as i64
                        && 0 <= (j as i64) + dj
                        && (j as i64) + dj < grid[0].len() as i64)
                    {
                        None
                    } else if let Some(l) =
                        grid[((i as i64) + di) as usize][((j as i64) + dj) as usize]
                    {
                        if l != curr + 1 {
                            None
                        } else {
                            Some((((i as i64) + di) as usize, ((j as i64) + dj) as usize))
                        }
                    } else {
                        None
                    }
                })
            })
            .collect()
    } else {
        vec![]
    }
}

fn parse(
    s: &str,
) -> (
    Vec<(usize, usize)>,
    HashMap<(usize, usize), Vec<(usize, usize)>>,
    Vec<(usize, usize)>,
    Vec<(usize, usize)>,
) {
    let mut edges = HashMap::new();
    let mut starts = vec![];
    let mut ends = vec![];
    let grid: Vec<Vec<Option<usize>>> = s
        .lines()
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.trim()
                .chars()
                .into_iter()
                .enumerate()
                .map(|(j, c)| {
                    if '0' <= c && c <= '9' {
                        if c == '9' {
                            ends.push((i, j));
                        }
                        Some((c as usize) - ('0' as usize))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    let nodes: Vec<(usize, usize)> = (0..grid.len())
        .into_iter()
        .flat_map(|i| (0..grid[0].len()).into_iter().map(move |j| (i, j)))
        .collect();
    nodes.iter().for_each(|node| {
        edges.insert(*node, vec![]);
    });
    s.lines().into_iter().enumerate().for_each(|(i, line)| {
        line.trim()
            .chars()
            .into_iter()
            .enumerate()
            .for_each(|(j, c)| {
                if c == '0' {
                    starts.push((i, j));
                }
                for nbor in neighbors(i, j, &grid) {
                    match edges.get_mut(&(i, j)) {
                        Some(v) => v.push(nbor),
                        None => (),
                    };
                }
            })
    });
    return (nodes, edges, starts, ends);
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let (nodes, edges, starts, ends) = parse(s);
            let graph = AdjacencyListGraph {
                nodes: &nodes,
                edges,
            };
            let result = starts
                .par_iter()
                .flat_map(|u| {
                    ends.par_iter()
                        .map(|v| if graph.dag_dfs(*u, *v) { 1 } else { 0 })
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let (nodes, edges, starts, ends) = parse(s);
            let graph = AdjacencyListGraph {
                nodes: &nodes,
                edges,
            };
            let result = starts
                .par_iter()
                .flat_map(|u| ends.par_iter().map(|v| graph.dag_count_dfs(*u, *v)))
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
