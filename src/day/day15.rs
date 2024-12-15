use super::super::stage::Stage;
use rayon::prelude::*;
use std::collections::HashSet;
pub use std::error::Error;

fn get_vec(c: char) -> Option<(i64, i64)> {
    match c {
        '>' => Some((1, 0)),
        '<' => Some((-1, 0)),
        '^' => Some((0, -1)),
        'v' => Some((0, 1)),
        _ => None,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Box(i64),
}

fn get_cell(c: char, cell_number: &mut i64) -> Cell {
    let cell = match c {
        '#' => Some(Cell::Wall),
        '.' => Some(Cell::Empty),
        '@' => Some(Cell::Empty),
        'O' => {
            *cell_number += 1;
            Some(Cell::Box(*cell_number))
        }
        _ => None,
    };
    cell.unwrap()
}

fn parse(s: &str) -> (Vec<Vec<Cell>>, Vec<(i64, i64)>, (i64, i64)) {
    let s: Vec<&str> = s.split("\n\n").collect();
    let mut cell_number: i64 = 0;
    let grid: Vec<Vec<Cell>> = s[0]
        .lines()
        .into_iter()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| get_cell(c, &mut cell_number))
                .collect()
        })
        .collect();
    let moves: Vec<&str> = s[1].lines().into_iter().map(|line| line.trim()).collect();
    let moves: Vec<(i64, i64)> = moves.join("").chars().filter_map(get_vec).collect();
    let robots: Vec<(usize, usize)> = s[0]
        .lines()
        .into_iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter_map(move |(j, c)| if c == '@' { Some((i, j)) } else { None })
        })
        .collect();
    let (x, y) = robots[0];
    let robot = (x as i64, y as i64);
    (grid, moves, robot)
}

fn parse2(s: &str) -> (Vec<Vec<Cell>>, Vec<(i64, i64)>, (i64, i64)) {
    let s: Vec<&str> = s.split("\n\n").collect();
    let mut cell_number: i64 = 0;
    let grid: Vec<Vec<Cell>> = s[0]
        .lines()
        .into_iter()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| get_cell(c, &mut cell_number))
                .collect()
        })
        .collect();
    let mut new_grid = vec![vec![Cell::Empty; grid[0].len() * 2]; grid.len()];
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| match c {
            Cell::Empty => {}
            Cell::Wall => {
                new_grid[i][2 * j] = Cell::Wall;
                new_grid[i][2 * j + 1] = Cell::Wall;
            }
            Cell::Box(b) => {
                new_grid[i][2 * j] = Cell::Box(*b);
                new_grid[i][2 * j + 1] = Cell::Box(*b);
            }
        })
    });
    let grid = new_grid;
    let moves: Vec<&str> = s[1].lines().into_iter().map(|line| line.trim()).collect();
    let moves: Vec<(i64, i64)> = moves.join("").chars().filter_map(get_vec).collect();
    let robots: Vec<(usize, usize)> = s[0]
        .lines()
        .into_iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter_map(move |(j, c)| if c == '@' { Some((i, j)) } else { None })
        })
        .collect();
    let (i, j) = robots[0];
    let robot = ((2 * j) as i64, i as i64);
    (grid, moves, robot)
}

#[allow(dead_code)]
fn print_grid2(grid: &Vec<Vec<Cell>>, (r_x, r_y): (i64, i64)) {
    let r_x = r_x as usize;
    let r_y = r_y as usize;
    grid.iter().enumerate().for_each(|(i, row)| {
        let row: String = row
            .iter()
            .enumerate()
            .map(|(j, c)| {
                if i == r_y && j == r_x {
                    '@'
                } else {
                    match c {
                        Cell::Box(b) => {
                            if j > 0 && grid[i][j - 1] == Cell::Box(*b) {
                                ']'
                            } else {
                                '['
                            }
                        }
                        Cell::Empty => '.',
                        Cell::Wall => '#',
                    }
                }
            })
            .collect();
        println!("{}", row);
    });
}

fn can_move(
    grid: &Vec<Vec<Cell>>,
    (x, y): (i64, i64),
    (dx, dy): (i64, i64),
    to_move: &mut HashSet<(i64, i64)>,
) -> bool {
    match grid[y as usize][x as usize] {
        Cell::Box(_) => {
            to_move.insert((x, y));
        }
        _ => {}
    };
    let x = x + dx;
    let y = y + dy;
    if !(0 <= x && x < grid[0].len() as i64 && 0 <= y && y < grid.len() as i64) {
        return false;
    }

    match grid[y as usize][x as usize] {
        Cell::Wall => false,
        Cell::Box(b) => {
            let neighboring_boxes = neighboring_boxes(grid, (x, y), b, dy);
            neighboring_boxes
                .map(|(x, y)| can_move(grid, (x, y), (dx, dy), to_move))
                .unwrap_or_else(|| true)
                && can_move(grid, (x, y), (dx, dy), to_move)
        }
        Cell::Empty => true,
    }
}

fn neighboring_boxes(
    grid: &Vec<Vec<Cell>>,
    (x, y): (i64, i64),
    box_num: i64,
    dy: i64,
) -> Option<(i64, i64)> {
    if dy == 0 {
        return None;
    }
    if grid[y as usize][(x - 1) as usize].eq(&Cell::Box(box_num)) {
        return Some((x - 1, y));
    } else if grid[y as usize][(x + 1) as usize].eq(&Cell::Box(box_num)) {
        return Some((x + 1, y));
    }
    None
}

fn process_move(mov: (i64, i64), grid: &mut Vec<Vec<Cell>>, robot: (i64, i64)) -> (i64, i64) {
    let mut to_move = HashSet::new();
    if can_move(grid, robot, mov, &mut to_move) {
        let mut new_grid = grid.clone();
        let (dx, dy) = mov;
        to_move.iter().for_each(|(x, y)| {
            new_grid[*y as usize][*x as usize] = Cell::Empty;
        });
        to_move.iter().for_each(|(x, y)| {
            new_grid[(y + dy) as usize][(x + dx) as usize] = grid[*y as usize][*x as usize];
        });
        let (x, y) = robot;
        new_grid[y as usize][x as usize] = Cell::Empty;
        *grid = new_grid;
        return (x + dx, y + dy);
    }
    robot
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let (mut grid, moves, mut robot) = parse(s);
            moves.iter().for_each(|mov| {
                robot = process_move(*mov, &mut grid, robot);
            });
            let result = grid
                .par_iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.par_iter().enumerate().map(move |(j, cell)| match cell {
                        Cell::Box(_) => i * 100 + j,
                        _ => 0,
                    })
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let (mut grid, moves, mut robot) = parse2(s);
            moves.iter().for_each(|mov| {
                robot = process_move(*mov, &mut grid, robot);
            });
            let actual_grid = grid.clone();
            let grid: Vec<Vec<Cell>> = grid
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(j, c)| match c {
                            Cell::Box(b) => {
                                if j > 0 && actual_grid[i][j - 1] == Cell::Box(*b) {
                                    Cell::Empty
                                } else {
                                    Cell::Box(*b)
                                }
                            }
                            c => c.clone(),
                        })
                        .collect()
                })
                .collect();
            let result = grid
                .iter()
                .enumerate()
                .flat_map(move |(i, row)| {
                    row.iter().enumerate().map(move |(j, cell)| match cell {
                        Cell::Box(_) => 100 * i + j,
                        _ => 0,
                    })
                })
                .reduce(|x, y| x + y)
                .unwrap_or_else(|| 0);
            Ok(result.to_string())
        }
    }
}
