use super::super::stage::Stage;
use im::ordset::OrdSet;
use rayon::prelude::*;
pub use std::error::Error;

fn parse(s: &str) -> Vec<usize> {
    (0..s.len() - 2)
        .into_par_iter()
        .filter_map(|i| if s[i..i + 3].eq("mul") { Some(i) } else { None })
        .collect()
}

enum ParsingStage {
    LParen,
    Left,
    Right,
}

fn parse_tuple(s: &str, i: usize) -> Option<(i32, i32)> {
    let mut idx = i + 3;
    let mut left = 0;
    let mut right = 0;
    let s = s.as_bytes();
    let mut parsing_stage = ParsingStage::LParen;
    while idx < s.len() {
        match parsing_stage {
            ParsingStage::LParen => {
                if s[idx] == ('(' as u8) {
                    parsing_stage = ParsingStage::Left;
                } else {
                    return None;
                }
            }
            ParsingStage::Left => {
                if '0' <= char::from(s[idx]) && char::from(s[idx]) <= '9' {
                    left = left * 10 + i32::from(s[idx] - ('0' as u8));
                } else if char::from(s[idx]) == ',' {
                    parsing_stage = ParsingStage::Right;
                } else {
                    return None;
                }
            }
            ParsingStage::Right => {
                if '0' <= char::from(s[idx]) && char::from(s[idx]) <= '9' {
                    right = right * 10 + i32::from(s[idx] - ('0' as u8));
                } else if char::from(s[idx]) == ')' {
                    return Some((left, right));
                } else {
                    return None;
                }
            }
        }
        idx += 1;
    }
    return None;
}

fn parse_dos(s: &str) -> Vec<usize> {
    (0..s.len() - 3)
        .into_par_iter()
        .filter_map(|i| {
            if s[i..i + 4].eq("do()") {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn parse_donts(s: &str) -> Vec<usize> {
    (0..s.len() - 7)
        .into_par_iter()
        .filter_map(|i| {
            if s[i..i + 7].eq("don't()") {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let muls = parse(s);
            let result = muls
                .par_iter()
                .map(|i| match parse_tuple(s, *i) {
                    Some((left, right)) => left * right,
                    None => 0,
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let muls = parse(s);
            let imuls: Vec<(usize, i32)> = muls
                .par_iter()
                .filter_map(|i| match parse_tuple(s, *i) {
                    Some((left, right)) => Some((*i, left * right)),
                    None => None,
                })
                .collect();
            let dos: Vec<usize> = parse_dos(s);
            let donts: Vec<usize> = parse_donts(s);
            let dos = OrdSet::from(dos);
            let donts = OrdSet::from(donts);

            let result = imuls
                .par_iter()
                .filter_map(|(i, prod)| match donts.get_prev(i) {
                    Some(x) => match dos.get_prev(i) {
                        Some(y) => {
                            if x < y {
                                Some(*prod)
                            } else {
                                None
                            }
                        }
                        None => None,
                    },
                    None => Some(*prod),
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
