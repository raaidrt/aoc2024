use super::super::stage::Stage;
use rayon::prelude::*;
pub use std::error::Error;

#[derive(Clone, Copy, Debug)]
struct Button {
    dx: i64,
    dy: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug)]
struct Game {
    a: Button,
    b: Button,
    prize: Position,
}

fn parse(s: &str) -> Vec<Game> {
    s.split("\n\n")
        .map(|game| {
            let mut button_a = Button { dx: 0, dy: 0 };
            let mut button_b = Button { dx: 0, dy: 0 };
            let mut prize = Position { x: 0, y: 0 };
            game.lines().into_iter().for_each(|line| {
                if line[..8].eq("Button A") {
                    let line: Vec<&str> = line[10..].trim().split(", ").collect();
                    button_a.dx = line[0][2..].parse().unwrap();
                    button_a.dy = line[1][2..].parse().unwrap();
                } else if line[..8].eq("Button B") {
                    let line: Vec<&str> = line[10..].trim().split(", ").collect();
                    button_b.dx = line[0][2..].parse().unwrap();
                    button_b.dy = line[1][2..].parse().unwrap();
                } else {
                    let line: Vec<&str> = line[7..].trim().split(", ").collect();
                    prize.x = line[0][2..].parse().unwrap();
                    prize.y = line[1][2..].parse().unwrap();
                }
            });
            Game {
                a: button_a,
                b: button_b,
                prize,
            }
        })
        .collect()
}

fn solve(game: &Game) -> Option<i64> {
    let result = (0..=100)
        .into_par_iter()
        .flat_map(|a_count| {
            (0..=100).into_par_iter().map(move |b_count| {
                let position = Position {
                    x: a_count * game.a.dx + b_count * game.b.dx,
                    y: a_count * game.a.dy + b_count * game.b.dy,
                };

                if position == game.prize {
                    a_count * 3 + b_count
                } else {
                    i64::max_value()
                }
            })
        })
        .reduce(|| i64::max_value(), |x, y| x.min(y));
    if result == i64::max_value() {
        None
    } else {
        Some(result)
    }
}

use fraction::GenericFraction;

fn not_parallel(game: &Game) -> bool {
    let div1: GenericFraction<i64> = GenericFraction::new(game.a.dx, game.a.dy);
    let div2: GenericFraction<i64> = GenericFraction::new(game.b.dx, game.b.dy);
    div1 != div2
}

const OFFSET: i64 = 10000000000000;

fn inv(a: i64, b: i64, c: i64, d: i64, x: i64, y: i64) -> Option<(i64, i64)> {
    let det = a * d - b * c;
    let m: GenericFraction<i64> = GenericFraction::new(d * x - b * y, det);
    let n: GenericFraction<i64> = GenericFraction::new(-c * x + a * y, det);
    if m.floor() == m && n.floor() == n {
        let m: i64 = m.numer().unwrap() * (m.denom().unwrap() / m.denom().unwrap().abs());
        let n: i64 = n.numer().unwrap() * (n.denom().unwrap() / n.denom().unwrap().abs());
        Some((m, n))
    } else {
        None
    }
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let games = parse(s);
            let result = games
                .par_iter()
                .filter_map(solve)
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let games = parse(s);
            let games: Vec<Game> = games
                .par_iter()
                .map(|game| Game {
                    prize: Position {
                        x: game.prize.x + OFFSET,
                        y: game.prize.y + OFFSET,
                    },
                    ..*game
                })
                .collect();

            assert!(games.par_iter().all(not_parallel));

            let result = games
                .par_iter()
                .filter_map(|game| {
                    inv(
                        game.a.dx,
                        game.b.dx,
                        game.a.dy,
                        game.b.dy,
                        game.prize.x,
                        game.prize.y,
                    )
                    .iter()
                    .flat_map(|(m, n)| {
                        if *m >= 0 && *n >= 0 {
                            Some(3 * m + n)
                        } else {
                            None
                        }
                    })
                    .next()
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
