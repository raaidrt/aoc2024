use super::super::stage::Stage;
use dashmap::DashMap;
use rayon::prelude::*;
pub use std::error::Error;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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

#[derive(Debug)]
struct AdjacencyListGraph<T: Hash + Eq + Clone> {
    nodes: Vec<T>,
    edges: HashMap<T, Vec<T>>,
}

trait Graph<T: Hash + Eq + Clone> {
    fn neighbors(&self, x: T) -> &Vec<T>;

    fn nodes(&self) -> &Vec<T>;

    fn dfs_topsort(&self, x: T, visited: &mut HashSet<T>, topsorted: &mut Vec<T>) {
        assert!(!visited.contains(&x));
        visited.insert(x.clone());
        for nbor in self.neighbors(x.clone()) {
            if !visited.contains(nbor) {
                self.dfs_topsort(nbor.clone(), visited, topsorted);
            }
        }
        topsorted.push(x);
    }

    fn topsort(&self) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut topsorted = vec![];
        self.nodes().into_iter().for_each(|node| {
            if !visited.contains(&node.clone()) {
                self.dfs_topsort(node.clone(), &mut visited, &mut topsorted);
            }
        });
        topsorted.reverse();
        topsorted
    }
}

impl<T: Hash + Eq + Clone> Graph<T> for AdjacencyListGraph<T> {
    fn nodes(&self) -> &Vec<T> {
        &self.nodes
    }
    fn neighbors(&self, x: T) -> &Vec<T> {
        self.edges.get(&x).unwrap()
    }
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

            let mut nodes = HashSet::new();
            let mut edges = HashMap::new();
            unordered.clone().into_iter().for_each(|line| {
                line.iter().for_each(|x| {
                    nodes.insert(*x);
                    edges.insert(*x, HashSet::new());
                });
            });
            orders.iter().for_each(|x| {
                nodes.insert(x.before);
                nodes.insert(x.after);
                let nbors = edges.get_mut(&x.before).unwrap();
                nbors.insert(x.after);
            });

            let result = unordered
                .iter()
                .map(|line| {
                    let mut line_set: HashSet<u32> = HashSet::new();
                    line.iter().for_each(|x| {
                        line_set.insert(*x);
                    });
                    let graph = AdjacencyListGraph {
                        nodes: nodes
                            .clone()
                            .into_iter()
                            .filter(|x| line_set.contains(x))
                            .collect(),
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
                .reduce(|x, y| x + y)
                .unwrap();

            Ok(result.to_string())
        }
    }
}
