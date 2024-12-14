// This file is auto-generated. Please edit build.rs to make changes
use std::error::Error;
use super::stage::Stage;
mod day01; pub mod day1 {pub use super::day01::run;}
mod day02; pub mod day2 {pub use super::day02::run;}
mod day03; pub mod day3 {pub use super::day03::run;}
mod day04; pub mod day4 {pub use super::day04::run;}
mod day05; pub mod day5 {pub use super::day05::run;}
mod day06; pub mod day6 {pub use super::day06::run;}
mod day07; pub mod day7 {pub use super::day07::run;}
mod day08; pub mod day8 {pub use super::day08::run;}
mod day09; pub mod day9 {pub use super::day09::run;}
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
use std::fmt;#[derive(Debug)]struct DayError(u8);impl fmt::Display for DayError {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f, "Invalid day: {}", self.0)}}impl Error for DayError {}
pub mod day15 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(15.into())))
                    }}
pub mod day16 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(16.into())))
                    }}
pub mod day17 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(17.into())))
                    }}
pub mod day18 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(18.into())))
                    }}
pub mod day19 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(19.into())))
                    }}
pub mod day20 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(20.into())))
                    }}
pub mod day21 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(21.into())))
                    }}
pub mod day22 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(22.into())))
                    }}
pub mod day23 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(23.into())))
                    }}
pub mod day24 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(24.into())))
                    }}
pub mod day25 {pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {Err(Box::new(super::DayError(25.into())))
                    }}
