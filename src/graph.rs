use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug)]
pub struct AdjacencyListGraph<'a, T: Hash + Eq + Clone> {
    pub nodes: &'a Vec<T>,
    pub edges: HashMap<T, Vec<T>>,
}

pub trait Graph<T: Hash + Eq + Clone> {
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
        self.nodes().iter().for_each(|node| {
            if !visited.contains(&node.clone()) {
                self.dfs_topsort(node.clone(), &mut visited, &mut topsorted);
            }
        });
        topsorted.reverse();
        topsorted
    }

    fn dag_dfs(&self, u: T, v: T) -> bool {
        if u == v {
            return true;
        }
        return self
            .neighbors(u)
            .iter()
            .map(|nbor| self.dag_dfs(nbor.clone(), v.clone()))
            .reduce(|x, y| x || y)
            .unwrap_or_else(|| false);
    }

    fn dag_count_dfs(&self, u: T, v: T) -> usize {
        if u == v {
            return 1;
        }
        return self
            .neighbors(u)
            .iter()
            .map(|nbor| self.dag_count_dfs(nbor.clone(), v.clone()))
            .reduce(|x, y| x + y)
            .unwrap_or_else(|| 0);
    }
}

impl<'a, T: Hash + Eq + Clone> Graph<T> for AdjacencyListGraph<'a, T> {
    fn nodes(&self) -> &Vec<T> {
        &self.nodes
    }
    fn neighbors(&self, x: T) -> &Vec<T> {
        self.edges.get(&x).unwrap()
    }
}
