// AI OPTIMIZED !! 

use std::collections::{VecDeque, HashSet};
use crate::utils;
//use nalgebra::DMatrix;
use good_lp::{variable, variables, Expression, SolverModel, Solution};
use good_lp::solvers::highs::highs;

pub fn solve(test: bool) -> Result<(), String> {
    let input = utils::read_lines(test, "dec10")?;
    //let mut total_min = 0u64;
    let mut total_button_presses = 0u64;

    for line in input {
        let (_pattern, schematics_etc) = line
            .split_once(' ')
            .ok_or("Invalid input format")?;
        
        // Target bitmask
        //let tgt = parse_target_mask(pattern);
 
        let mut schematics_etc = schematics_etc.split_whitespace().collect::<Vec<&str>>();
        let tgt_joltages = schematics_etc.pop().unwrap()
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|s| s.parse::<u64>().ok().unwrap())
            .rev()
            .collect::<Vec<u64>>();
        
        let size = tgt_joltages.len();
        let schematics_nums: Vec<Vec<u64>> = schematics_etc
            .into_iter()
            .map(parse_switch_mask)
            .map(|num| convert_to_binary_vector(num, size))
            .collect();

        let min_button_presses = solve_min_sum(
                &schematics_nums, &tgt_joltages)
            .unwrap();

        println!("Soln: {:?}", min_button_presses);
        total_button_presses += min_button_presses.iter().sum::<u64>();

        //let min_steps = find_shortest_path_bfs(tgt, &schematics_nums);
        
        //println!("Tgt: {:b} => Steps: {}", tgt, min_steps);
        //total_min += min_steps as u64;
    }

    //println!("Total minimum switch toggles: {}", total_min);
    println!("Total min button presses needed: {}", total_button_presses);
    Ok(())
}

/// Uses BFS to find the minimum number of switches to reach the target bitmask.
fn _find_shortest_path_bfs(target: u64, switches: &[u64]) -> usize {
    if target == 0 { return 0; }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Start with 0 toggles (all lights off)
    queue.push_back((0u64, 0usize)); 
    visited.insert(0u64);

    while let Some((current_mask, steps)) = queue.pop_front() {
        for &switch in switches {
            let next_mask = current_mask ^ switch;

            if next_mask == target {
                return steps + 1;
            }

            if !visited.contains(&next_mask) {
                visited.insert(next_mask);
                queue.push_back((next_mask, steps + 1));
            }
        }
    }
    0 // Return 0 if no path is found
}

fn _parse_target_mask(pattern: &str) -> u64 {
    let inner = pattern.trim_matches(|c| c == '[' || c == ']');
    inner.chars().enumerate().fold(0u64, |acc, (i, c)| {
        if c == '#' { acc | (1 << i) } else { acc }
    })
}

fn parse_switch_mask(s: &str) -> u64 {
    let inner = s.trim_matches(|c| c == '(' || c == ')' || c == '{' || c == '}');
    inner.split(',')
        .filter_map(|n| n.trim().parse::<u32>().ok())
        .fold(0u64, |acc, bit| acc | (1 << bit))
}

fn convert_to_binary_vector( num: u64, size: usize ) -> Vec<u64> {
    (0..size)
        .rev() // Start from the most significant bit to maintain standard reading order
        .map(|i| {
            // Check if i is within u64 bounds to avoid overflow/panic on shift
            if i < 64 {
                (num >> i) & 1
            } else {
                0 // Padding with zeros if size > 64
            }
        })
        .collect()
}

fn solve_min_sum(
    matrix_v_v: &Vec<Vec<u64>>, 
    vector_b: &Vec<u64>
) -> Result<Vec<u64>, String> {
    
    let cols = matrix_v_v.len(); 
    let rows = if cols > 0 { matrix_v_v[0].len() } else { return Err("Empty matrix".into()); };

    if vector_b.len() != rows {
        return Err(format!("Target vector size ({}) must match matrix row count ({})", vector_b.len(), rows));
    }

    let mut vars = variables!();
    
    // Create one variable for each column (each inner vector represents a column now)
    let x_vars: Vec<_> = (0..cols)
        .map(|_| vars.add(variable().integer().min(0.0)))
        .collect();

    let total_sum: Expression = x_vars.iter().sum();
    let mut model = vars.minimise(total_sum).using(highs);

    // Build constraints: Sum(Matrix[c][r] * x[c]) = b[r]
    for r in 0..rows {
        let mut row_expr = Expression::from(0.0);
        for c in 0..cols {
            if matrix_v_v[c][r] == 1 {
                row_expr += x_vars[c];
            } else if matrix_v_v[c][r] > 0 {
                row_expr += x_vars[c] * (matrix_v_v[c][r] as f64);
            }
        }
        model = model.with(row_expr.eq(vector_b[r] as f64));
    }

    match model.solve() {
        Ok(solution) => {
            let result = x_vars.iter()
                .map(|&v| solution.value(v).round() as u64)
                .collect();
            Ok(result)
        }
        Err(_) => Err("No valid integer solution exists".into()),
    }
}

// MY SUPER CRAZY ATTEMPT >> 

// use crate::utils;

// pub fn solve( test: bool ) -> Result<(), String> { 
//     let input = utils::read_lines(test, "dec10")?;
//     let mut total_min = 0u64; 
//     for line in input { 
//         let (pattern, schematics_etc) = line.split_once(' ').unwrap();
//         let mut schematics = schematics_etc.split(' ').collect::<Vec<&str>>();
//         let _joltage = schematics.pop().unwrap_or_default();
//         let tgt = get_num(pattern);
//         let schematics_nums = schematics.iter()
//             .map(|s| {
//                 let c = s.strip_prefix('(').unwrap();
//                 let c = c.strip_suffix(')').unwrap();
//                 get_num2(pattern.len()-2, c)
//             })
//             .collect::<Vec<u64>>();
//         let min = get_shortest_path(tgt, &schematics_nums);
//         println!("Tgt: {tgt:?} / {schematics_nums:?} => {}", min.len());
//         total_min += min.len() as u64;
//     }
//     println!("Total minimum switch toggles: {}", total_min);
//     Ok(())
// }

// fn get_num2( n: usize, pattern: &str ) -> u64 {
//     let s = pattern.split(',')
//         .map(|c| c.parse::<usize>().ok().unwrap())
//         .collect::<Vec<usize>>();
//     let mut res = 0u64;
//     for i in s {
//         let m = (n-i-1) as u32;
//         res += 2u64.pow(m);
//     };
//     res
// }

// fn get_num( pattern: &str ) -> u64 { 
//     let res = pattern
//         .strip_prefix('[').and_then(|s| s.strip_suffix(']'))
//         .map(|s| s.chars()
//             .rev().enumerate()
//             .map(|(i, c)| 2u64.pow(i as u32)*(if c == '.' {0} else {1}) )
//             .sum::<u64>());
//     res.unwrap_or_default()
// }

// fn get_shortest_path( tgt: u64, schematics: &Vec<u64> ) -> Vec<u64> { 
//     let mut paths = Vec::<Vec<u64>>::new(); 
//     for (i, n) in schematics.iter().enumerate() { 
//         let s1 = &schematics[0..i];
//         let s2 = &schematics[i+1..schematics.len()];
//         let s = [s1, s2].concat();
//         let path = get_path(tgt, n, s);
//         paths.push(path);
//     };
//     let s = paths.iter().min_by_key(|v| v.len());
//     match s { 
//         Some(c) => (*c).clone(),
//         None => Vec::<u64>::new(),
//     }
// }

// fn get_path( tgt: u64, n: &u64, s: Vec<u64> ) -> Vec<u64> { 
//     let mut res = Vec::from([*n]);
//     if tgt != *n { 
//         let m = tgt^n;
//         let mut path = get_shortest_path(m, &s);
//         res.append(&mut path);
//     };
//     res
// }

// #[cfg(test)]

// mod tests { 
//     use super::*;

//     #[test]
//     fn test_basics() { 
//         let v = Vec::<Vec<u64>>::new();
//         assert_eq!(v.iter().min_by_key(|x| x.len()), None);
//     }


//     #[test]
//     fn test_get_num2() { 
//         assert_eq!(get_num2(4, "1,3"), 5); 
//         assert_eq!(get_num2(4, "3"), 1); 
//         assert_eq!(get_num2(4, "2,3"), 3); 
//     }

// }
