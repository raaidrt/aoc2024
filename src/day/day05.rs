use super::super::stage::Stage;
use crate::graph::{AdjacencyListGraph, Graph};
use dashmap::{DashMap, DashSet};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
pub use std::error::Error;

#[derive(Debug, Clone, Copy)]
struct Order {
    before: u32,
    after: u32,
}

fn parse(s: &str) -> (Vec<Order>, Vec<Vec<u32>>) {
    let lines: Vec<&str> = s.lines().into_iter().map(|s| s.trim()).collect();
    let (i, _) = lines
        .iter()
        .enumerate()
        .find(|(_, s)| s.is_empty())
        .unwrap();
    let orders: Vec<Order> = lines[..i]
        .par_iter()
        .map(|s| {
            let order: Vec<u32> = s
                .split("|")
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect();
            Order {
                before: order[0],
                after: order[1],
            }
        })
        .collect();
    let lines = lines[i + 1..]
        .par_iter()
        .map(|s| {
            s.split(",")
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    (orders, lines)
}

fn is_correct(orders: &Vec<Order>, line: &Vec<u32>) -> bool {
    let map = DashMap::new();
    line.into_par_iter().enumerate().for_each(|(i, x)| {
        map.insert(*x, i);
    });
    orders
        .par_iter()
        .map(
            |order| match (map.get(&order.before), map.get(&order.after)) {
                (None, _) => true,
                (_, None) => true,
                (Some(i), Some(j)) => {
                    if *i.value() < *j.value() {
                        true
                    } else {
                        false
                    }
                }
            },
        )
        .reduce(|| true, |x, y| x && y)
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let (orders, lines) = parse(s);
            let result = lines
                .par_iter()
                .map(|line| {
                    if is_correct(&orders, line) {
                        line[line.len() / 2]
                    } else {
                        0
                    }
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let (orders, lines) = parse(s);
            let unordered: Vec<Vec<u32>> = lines
                .into_par_iter()
                .filter(|line| !is_correct(&orders, line))
                .collect();

            let nodes = DashSet::new();
            let edges = DashMap::new();
            unordered.par_iter().for_each(|line| {
                line.iter().for_each(|x| {
                    nodes.insert(*x);
                    edges.insert(*x, HashSet::new());
                });
            });
            orders.par_iter().for_each(|x| {
                nodes.insert(x.before);
                nodes.insert(x.after);
                (*edges.get_mut(&x.before).unwrap()).insert(x.after);
            });

            let result = unordered
                .par_iter()
                .map(|line| {
                    let mut line_set: HashSet<u32> = HashSet::new();
                    line.iter().for_each(|x| {
                        line_set.insert(*x);
                    });
                    let nodes: Vec<u32> = nodes
                        .iter()
                        .filter(|x| line_set.contains(x))
                        .map(|x| x.clone())
                        .collect();
                    let graph = AdjacencyListGraph {
                        nodes: &nodes,
                        edges: edges
                            .clone()
                            .into_iter()
                            .filter(|(k, _)| line_set.contains(k))
                            .map(|(k, v)| {
                                (k, v.into_iter().filter(|x| line_set.contains(x)).collect())
                            })
                            .collect(),
                    };
                    let topsort = graph.topsort();
                    assert!(topsort.len() == graph.nodes().len());
                    let mut order = HashMap::new();
                    topsort.iter().enumerate().for_each(|(i, x)| {
                        order.insert(x, i);
                    });
                    let mut line: Vec<u32> = line.into_iter().map(|x| *x).collect();
                    line.sort_by(|a, b| (order.get(a).unwrap().cmp(order.get(b).unwrap())));
                    line[line.len() / 2]
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
