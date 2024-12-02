use super::super::stage::Stage;
use rayon::prelude::*;
pub use std::error::Error;

fn parse(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .into_iter()
        .map(|line| line.trim().split(" ").map(|x| x.parse().unwrap()).collect())
        .collect()
}

enum SeqOrder {
    Increasing,
    Decreasing,
}

fn check_safety(line: &Vec<i32>, seq_order: SeqOrder, exclude: Option<usize>) -> bool {
    (0..line.len() - 1)
        .into_par_iter()
        .map(|i| {
            let mut next_idx = i + 1;
            if let Some(j) = exclude {
                if i == j || (i + 1 == j && i + 1 == line.len() - 1) {
                    return 1;
                } else if i + 1 == j {
                    next_idx = i + 2;
                }
            }
            if (match seq_order {
                SeqOrder::Increasing => line[i] < line[next_idx],
                SeqOrder::Decreasing => line[i] > line[next_idx],
            }) && 1 <= (line[next_idx] - line[i]).abs()
                && (line[next_idx] - line[i]).abs() <= 3
            {
                1
            } else {
                0
            }
        })
        .reduce(|| 1, |x, y| x * y)
        > 0
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let nums = parse(s);
            let result = nums
                .par_iter()
                .map(|line| {
                    check_safety(&line, SeqOrder::Increasing, None)
                        | check_safety(&line, SeqOrder::Decreasing, None)
                })
                .map(|b| if b { 1 } else { 0 })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let nums = parse(s);
            let result = nums
                .par_iter()
                .map(|line| {
                    let orig_res = check_safety(&line, SeqOrder::Increasing, None)
                        || check_safety(&line, SeqOrder::Decreasing, None);
                    let new_res = (0..line.len())
                        .into_par_iter()
                        .map(|j| {
                            check_safety(&line, SeqOrder::Increasing, Some(j))
                                || check_safety(&line, SeqOrder::Decreasing, Some(j))
                        })
                        .reduce(|| false, |x, y| x || y);
                    orig_res || new_res
                })
                .map(|b| if b { 1 } else { 0 })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
