use super::super::stage::Stage;
use im::OrdMap;
use rayon::prelude::*;
pub use std::error::Error;

#[derive(Debug, Copy, Clone)]
struct File {
    index: usize,
    position: usize,
}

#[derive(Debug, Clone)]
struct FileSystem {
    free: Vec<usize>,
    occupied: Vec<File>,
}

#[derive(Debug, Copy, Clone)]
struct FileBlock {
    index: usize,
    size: usize,
    position: usize,
}

#[derive(Debug, Copy, Clone)]
struct FreeBlock {
    size: usize,
    position: usize,
}

#[derive(Debug, Clone)]
struct BlockFileSystem {
    free: Vec<FreeBlock>,
    occupied: Vec<FileBlock>,
}

fn parse(s: &str) -> FileSystem {
    let mut free = vec![];
    let mut occupied = vec![];
    let mut counter: usize = 0;
    s.trim().chars().into_iter().enumerate().for_each(|(i, c)| {
        let freq: usize = (c as usize) - ('0' as usize);
        if i % 2 == 1 {
            (0..freq).into_iter().for_each(|f| {
                free.push(counter + f);
            });
        } else {
            (0..freq).into_iter().for_each(|f| {
                occupied.push(File {
                    index: i / 2,
                    position: f + counter,
                });
            });
        }
        counter += freq;
    });
    FileSystem { free, occupied }
}

fn parse_block(s: &str) -> BlockFileSystem {
    let mut free = vec![];
    let mut occupied = vec![];
    let mut counter: usize = 0;
    s.trim().chars().into_iter().enumerate().for_each(|(i, c)| {
        let freq: usize = (c as usize) - ('0' as usize);
        if i % 2 == 1 {
            free.push(FreeBlock {
                position: counter,
                size: freq,
            });
        } else {
            occupied.push(FileBlock {
                position: counter,
                size: freq,
                index: i / 2,
            });
        }
        counter += freq;
    });
    BlockFileSystem { free, occupied }
}

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let file_system = parse(s);
            let free = file_system.free;
            let mut occupied = file_system.occupied;
            let mut free_ptr = 0;
            for i in (0..occupied.len()).rev() {
                if free[free_ptr] >= occupied[i].position {
                    // println!("stopping at index {i} with file {:?}", occupied[i]);
                    break;
                }
                occupied[i].position = free[free_ptr];
                free_ptr += 1;
            }
            occupied.sort_by(|x, y| x.position.cmp(&y.position));
            let result = occupied
                .par_iter()
                .map(|file| file.position * file.index)
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
        Stage::B => {
            let file_system = parse_block(s);
            let mut free = file_system.free;
            let mut occupied = file_system.occupied;
            for i in (0..occupied.len()).rev() {
                for free_i in 0..free.len() {
                    if free[free_i].position >= occupied[i].position {
                        break;
                    }
                    if free[free_i].size < occupied[i].size {
                        continue;
                    }
                    let original_position = free[free_i].position;
                    free[free_i].size -= occupied[i].size;
                    free[free_i].position += occupied[i].size;
                    occupied[i].position = original_position;
                }
            }
            let result = occupied
                .par_iter()
                .flat_map(|o| {
                    (o.position..o.position + o.size)
                        .into_par_iter()
                        .map(|i| i * o.index)
                })
                .reduce(|| 0, |x, y| x + y);
            Ok(result.to_string())
        }
    }
}
