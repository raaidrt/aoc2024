use super::super::stage::Stage;
use im::HashSet;
use rayon::prelude::*;
pub use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Cell {
    Open,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
struct Guard {
    i: i64,
    j: i64,
    dir: Direction,
}

impl Guard {
    fn dir(&self) -> (i64, i64) {
        match self.dir {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn_right(&self) -> Guard {
        let dir = match self.dir {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        };
        Guard {
            dir,
            i: self.i,
            j: self.j,
        }
    }

    fn advance(&self) -> Guard {
        let (di, dj) = self.dir();
        Guard {
            i: self.i + di,
            j: self.j + dj,
            dir: self.dir,
        }
    }
}

fn parse_dir(c: char) -> Option<Direction> {
    match c {
        '>' => Some(Direction::Right),
        '<' => Some(Direction::Left),
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        _ => None,
    }
}

fn parse(s: &str) -> (Vec<Vec<Cell>>, Guard) {
    let grid = s
        .lines()
        .into_iter()
        .map(|line| {
            line.trim()
                .as_bytes()
                .to_vec()
                .iter()
                .map(|b| {
                    if *b == '#' as u8 {
                        Cell::Blocked
                    } else {
                        Cell::Open
                    }
                })
                .collect()
        })
        .collect();
    let guards: Vec<Guard> = s
        .lines()
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let line: Vec<Guard> = line
                .trim()
                .as_bytes()
                .to_vec()
                .iter()
                .map(|c| *c as char)
                .enumerate()
                .filter_map(|(j, c)| {
                    parse_dir(c).map(|dir| Guard {
                        i: i as i64,
                        j: j as i64,
                        dir,
                    })
                })
                .collect();
            line
        })
        .flatten()
        .collect();
    (grid, guards[0])
}

fn in_bounds(grid: &Vec<Vec<Cell>>, guard: &Guard) -> bool {
    0 <= guard.i && guard.i < grid.len() as i64 && 0 <= guard.j && guard.j < grid[0].len() as i64
}

fn collision(grid: &Vec<Vec<Cell>>, guard: &Guard) -> bool {
    if !in_bounds(grid, guard) {
        false
    } else {
        grid[guard.i as usize][guard.j as usize] == Cell::Blocked
    }
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let (grid, mut guard) = parse(s);
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
            while in_bounds(&grid, &guard) {
                visited[guard.i as usize][guard.j as usize] = true;
                let new_guard = guard.advance();
                if collision(&grid, &new_guard) {
                    guard = guard.turn_right();
                } else {
                    guard = new_guard;
                }
            }

            let result = visited
                .par_iter()
                .map(|line| {
                    line.par_iter()
                        .map(|b| if *b { 1 } else { 0 })
                        .reduce(|| 0, |x, y| x + y)
                })
                .reduce(|| 0, |x, y| x + y);

            Ok(result.to_string())
        }
        Stage::B => {
            let (grid, guard) = parse(s);

            let blockages: Vec<(usize, usize)> = (0..grid.len())
                .map(|i| (0..grid[0].len()).into_iter().map(move |j| (i, j)))
                .flatten()
                .collect();
            let result = blockages
                .par_iter()
                .map(|(i, j)| {
                    if *i as i64 == guard.i && *j as i64 == guard.j || grid[*i][*j] == Cell::Blocked
                    {
                        return 0;
                    }
                    let mut guard = guard.clone();
                    let mut prevs: HashSet<Guard> = HashSet::new();

                    while in_bounds(&grid, &guard) && !prevs.contains(&guard) {
                        prevs.insert(guard);
                        let new_guard = guard.advance();
                        if collision(&grid, &new_guard)
                            || (new_guard.i == *i as i64 && new_guard.j == *j as i64)
                        {
                            guard = guard.turn_right();
                        } else {
                            guard = new_guard;
                        }
                    }
                    if prevs.contains(&guard) {
                        1
                    } else {
                        0
                    }
                })
                .reduce(|| 0, |x, y| x + y);

            Ok(result.to_string())
        }
    }
}
