use super::super::stage::Stage;
use rayon::prelude::*;
pub use std::error::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Robot {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

fn parse(s: &str) -> Vec<Robot> {
    s.lines()
        .into_iter()
        .map(|line| {
            let (x, y, dx, dy) = scan_fmt!(line, "p={},{} v={},{}", i64, i64, i64, i64).unwrap();
            Robot { x, y, dx, dy }
        })
        .collect()
}

const BOUND_X: i64 = 101;
const BOUND_Y: i64 = 103;

fn pos_mod(x: i64, y: i64) -> i64 {
    let modulus = x % y;
    if modulus < 0 {
        y + modulus
    } else {
        modulus
    }
}

fn simulate(robot: &Robot, steps: i64) -> (i64, i64) {
    (
        pos_mod(robot.x + robot.dx * steps, BOUND_X),
        pos_mod(robot.y + robot.dy * steps, BOUND_Y),
    )
}

fn robot_in_quadrant(
    qx: i64,
    qy: i64,
    bound_length_x: i64,
    bound_length_y: i64,
    x: i64,
    y: i64,
) -> bool {
    let lower_bound_x = qx * (bound_length_x + 1);
    let x_in_bounds = qx * lower_bound_x <= x && x < (qx * lower_bound_x) + bound_length_x;
    let lower_bound_y = qy * (bound_length_y + 1);
    let y_in_bounds = qy * lower_bound_y <= y && y < (qy * lower_bound_y) + bound_length_y;
    x_in_bounds && y_in_bounds
}

fn print_robots(robots: &Vec<Robot>) -> String {
    let mut grid = vec![vec![0; BOUND_X as usize]; BOUND_Y as usize];
    robots.iter().for_each(|robot| {
        grid[robot.y as usize][robot.x as usize] += 1;
    });
    let rows: Vec<String> = grid
        .iter()
        .map(|row| {
            let result: Vec<String> = row
                .iter()
                .map(|x| {
                    if *x == 0 {
                        " ".to_string()
                    } else {
                        x.to_string()
                    }
                })
                .collect();
            result.join("")
        })
        .collect();
    rows.join("\n")
}

const CHRISTMAS: &str = "                             1                                        1                         1    
1            1                                   1             1                     1               
                   1               1                                                                 
                                                                                       1             
                                                                        1                            
     1                                                                                 1             
                                                                                                     
                            1                   1                                                    
                                                                                     1               
                                                                                                     
            1    1                                                                           1       
                                                          1 1                     1                  
                                                                      1                    1         
                               1                                                                     
                                                          1                                          
                                      1          1                            1                      
                1                                                                                    
             1                                    1                                                  
                                                                                               1     
                              1                                              1          1          1 
                                                 1    1                                              
 1                                                                                                   
                                                                                                     
1                                                                                                    
                                      1111111111111111111111111111111                                
                                      1                             1                                
                                      1                             1                                
  11                                  1                             1                                
                                      1                             1                                
     1                                1              1              1                                
                                      1             111             1     1                          
                                      1            11111            1   1                            
                                      1           1111111           1      1                         
                                      1          111111111          1                                
                                      1            11111            1                                
                         1            1           1111111           1 1                  1           
1                                   1 1          111111111          1                      1 1       
                                      1         11111111111         1                                
                                      1        1111111111111        1                                
           1                          1          111111111          1     1         1             1  
                                      1         11111111111         1                                
                                      1        1111111111111        1              1                 
                                      1       111111111111111       1 1                              
                         1            1      11111111111111111      1                                
                                      1        1111111111111        1             1                  
                                   1  1       111111111111111       1   1                            
                                      1      11111111111111111      1       11                       
                                      1     1111111111111111111     1                                
                        1             1    111111111111111111111    1                             1  
          1                  1        1             111             1                                
 1                                    1             111             1                        1       
       1                              1             111             1              1                 
                                      1                             1                                
                              1       1                             1                                
                                      1                             1 1                              
                                    1 1                             1       1                        
1                                     1111111111111111111111111111111                                
                                                                                                     
                                                       11                                       1    
   1                                                                                                 
                       1                   1                            1      1                     
                                                                                                     
    1                                                       1         1                             1
       1                            1                            1            1                      
                                                                                                     
              1                                    1                                                 
                                               1                                   1                 
                                                             1                                       
                           1                                                          1              
     1                                          1  1                                                1
    1                                                                                                
                                                                                                     
                                                                                                     
                                                           1            11   1                       
                                   1                                              1                  
                                                                                                     
                                                                                                     
                                                                                                    1
                                                                                                     
                                      1                                                              
                                                                                                     
                                               1            1                                        
 11                                                                  1                               
                                                                                                     
                                                                                          1          
                   1      1                1                                                         
                    1                                                     1                          
                                                                                                     
        1            1         1                                                                     
                                          1                               1                          
                                                                                                     
                                     1                                                               
                                                                                                     
             1     1                                                                                 
                               1                                                        1            
                            1                                                                        
                                                                                                     
          1                                                                                          
                                1                                   1                                
                 1   1                                                                               
                                                                                         1           
                                                                                                     
                      1                         1   1                                                ";

pub fn run(s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
    match stage {
        Stage::A => {
            let robots = parse(s);
            let num_steps = 100;

            let robots: Vec<(i64, i64)> = robots
                .par_iter()
                .map(|robot| simulate(robot, num_steps))
                .collect();

            let bound_length_x = BOUND_X / 2;
            let bound_length_y = BOUND_Y / 2;

            let result: Vec<usize> = [0, 1]
                .par_iter()
                .flat_map(|qx| {
                    [0, 1].par_iter().map(|qy| {
                        robots
                            .par_iter()
                            .filter(|robot| {
                                robot_in_quadrant(
                                    qx.clone(),
                                    qy.clone(),
                                    bound_length_x,
                                    bound_length_y,
                                    robot.0,
                                    robot.1,
                                )
                            })
                            .count()
                    })
                })
                .collect();
            let result = result
                .into_par_iter()
                .reduce(|| 1, |x, y| x.clone() * y.clone());
            Ok(result.to_string())
        }
        Stage::B => {
            let mut robots = parse(s);
            let mut step = 0;
            for num_steps in 0..2000000 {
                robots = robots
                    .par_iter()
                    .map(|robot| {
                        let (x, y) = simulate(robot, 1);
                        Robot { x, y, ..*robot }
                    })
                    .collect();

                if CHRISTMAS.eq(&print_robots(&robots)) {
                    step = num_steps + 1;
                    break;
                }
            }
            let result = step;
            Ok(result.to_string())
        }
    }
}
