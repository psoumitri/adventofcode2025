
use std::collections::{HashMap, HashSet};

use crate::utils;

pub fn solve( test: bool ) -> Result<(), String> { 
    
    let input = utils::read_lines(test, "dec08")?;
    let jn_locations : Vec<Vec<u64>> = input.iter().map(|i|
        i.split(',').map(|s| s.parse::<u64>()
                .unwrap_or_else(|_e| 0))
            .collect::<Vec::<u64>>()
    ).collect();
    let mut circuits = jn_locations.iter()
        .map(|c| HashSet::from([c])).collect::<Vec<HashSet<&Vec<u64>>>>();
    let mut jn_distances = get_jn_distances(&jn_locations);
    //let n = if test { 10 } else { 1000 };
    //for _i in 0..n { 
    while circuits.len() > 1 {
        let (&closest_pair, &_dist) 
            = jn_distances.iter()
            .min_by(|a,b| 
                a.1.partial_cmp(b.1).unwrap())
            .unwrap();
        let (jn1, jn2) = closest_pair;
        jn_distances.remove(&closest_pair);

        // amend circuits to add to the right set, removing the individual sets.
        let s_jn1 = extract(&mut circuits, jn1);
        let s_jn2 = extract(&mut circuits, jn2);
        let joined = s_jn1.union(&s_jn2).cloned().collect();
        circuits.push(joined);
        if circuits.len() == 1 { 
            let x = jn1[0]*jn2[0];
            println!("Final joiner:: {}", x);
        }
    }
    /*
    let mut c_lens = circuits.iter().map(
        |c| c.len()
    ).collect::<Vec<usize>>();

    c_lens.sort_by(|a,b| b.cmp(a));
    let f = c_lens.into_iter().take(3).collect::<Vec<usize>>();
    let p = f.iter().product::<usize>();
    println!("top 3: {:?}, product: {}", f, p); 
    */
    Ok(())
}

fn extract<'a>(circuits: &mut Vec<HashSet<&'a Vec<u64>>>, t: &Vec<u64>) 
    -> HashSet<&'a Vec<u64>> {
    circuits.iter()
        .position(|c| c.contains(t))
        .map(|m| circuits.swap_remove(m))
        .unwrap_or_default()
}


fn get_jn_distances(jn_locations: &Vec<Vec<u64>>) -> HashMap<(&Vec<u64>, &Vec<u64>), f64> {
    let mut jn_distances 
        = HashMap::<(&Vec<u64>, &Vec<u64>), f64>::new();

    for i in 0..(jn_locations.len()-1) { 
        let src = &jn_locations[i];
        for j in i+1..jn_locations.len() { 
            let dest = &jn_locations[j];
            let dist = src.iter().zip(dest)
                .map(|(a,b)| a.abs_diff(*b).pow(2) as f64)
                .sum::<f64>()
                .sqrt();
            jn_distances.insert((&src, &dest), dist);
        }
    };

    jn_distances
}

#[cfg(test)]


mod tests { 
    use super::*;

    #[test]
    fn test_basics() { 
        let a = HashMap::from([
            ("Mercury", 0.4),
            ("Venus", 0.1)
        ]);
        let min_entry = a.iter()
            .min_by(|a,b| a.1.partial_cmp(b.1).unwrap());
        println!("{min_entry:?}");

        let b = HashSet::from(['a', 'b']);
        assert_eq!(b.contains(&'a'), true);
    }
}