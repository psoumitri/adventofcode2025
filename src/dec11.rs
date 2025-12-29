use crate::utils;
use std::collections::{HashMap, HashSet, VecDeque};

// Use &str to avoid heap allocations during traversal
fn count_paths(
    nodes: &HashSet::<String>, 
    edges: &[(String, String)], 
    start: &str, 
    end: &str,
    ignore: &[&str],
) -> u64 { 
    let mut adj = HashMap::<&str, Vec<&str>>::with_capacity(nodes.len());
    let mut in_degree = HashMap::<&str, u64>::with_capacity(nodes.len());
    let mut path_counts = HashMap::<&str, u64>::with_capacity(nodes.len());
    
    for node in nodes { 
        in_degree.insert(node.as_str(), 0);
        path_counts.insert(node.as_str(), if node == start { 1 } else { 0 });
    }
    
    for (u, v) in edges { 
        if ignore.contains(&v.as_str()) || ignore.contains(&u.as_str()) { continue; }
        adj.entry(u.as_str()).or_default().push(v.as_str());
        in_degree.entry(v.as_str()).and_modify(|e| *e += 1);
    }

    // Kahn's Algorithm
    let mut queue: VecDeque<&str> = in_degree.iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&node, _)| node)
        .collect();
    
    while let Some(node) = queue.pop_front() { 
        let count = *path_counts.get(node).unwrap_or(&0);
        
        // Use .get().iter().flatten() to safely handle nodes with no neighbors
        if let Some(neighbors) = adj.get(node) {
            for &n in neighbors { 
                
                // Update path count
                path_counts.entry(n)
                    .and_modify(|e| *e += count);
                
                // Update in-degree
                if let Some(deg) = in_degree.get_mut(n) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(n);
                    }
                }
            }
        }
    }
    
    path_counts.get(end).copied().unwrap_or(0)
}

pub fn solve(test: bool) -> Result<(), String> { 
    let input = utils::read_lines(test, "dec11")?;
    let mut edges = Vec::<(String, String)>::new();
    let mut nodes = HashSet::from(["out".to_string()]);
    for line in input { 
        let (src, tgts) = line.split_once(' ').unwrap();
        let src = src.strip_suffix(':').unwrap();
        nodes.insert(src.to_string());
        let tgts = tgts.split_whitespace().collect::<Vec<&str>>();
        tgts.iter().for_each(|t| { 
            edges.push((src.to_string(), t.to_string()));
            nodes.insert(t.to_string());
        });
    }
    let total = count_paths(&nodes, &edges, "svr", "out", &[]);
    println!("Total Paths :: {}", total);
    let with_dac = total - count_paths(&nodes, &edges, "svr", "out", &["dac"]);
    let with_fft = total - count_paths(&nodes, &edges, "svr", "out", &["fft"]);
    let neither = count_paths(&nodes, &edges, "svr", "out", &["dac", "fft"]);
    let both = with_dac + with_fft - ( total - neither );
    println!("Paths with both a `dac` and a `fft` :: {}", both);
    Ok(())
}