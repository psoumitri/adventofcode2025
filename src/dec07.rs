use std::collections::HashMap;

use crate::utils;

fn is_below_opener( location: (u64, u64), 
    opener: (u64, u64),
    splitters: &HashMap<(u64, u64), bool> ) -> bool { 

    opener.1 == location.1 && (0..location.0)
        .find(|&x| splitters.contains_key(&(x,location.1))) == None 
}

fn splits_beam( location: (u64, u64), 
        opener: (u64, u64),
        splitters: &HashMap<(u64, u64), bool> ) -> bool { 
    
    if is_below_opener(location, opener, splitters) { 
        println!("Splitter at: {location:?} sits unhindered below opener at: {opener:?}");
        return true;
    }
    
    let find_splitter = |i| { 
        (0..location.0).rev()
        .find(|&x| splitters.contains_key(&(x,i))) };

    let l_splitter = find_splitter(location.1-1);
    let m_splitter = find_splitter(location.1);
    let r_splitter = find_splitter(location.1+1);

    let has_good_ls = l_splitter
        .and_then(|l| 
            m_splitter.and_then(|m| Some(l>m)).or(Some(true)) ) == Some(true);
    let has_good_rs = r_splitter
        .and_then(|r| 
            m_splitter.and_then(|m| Some(r>m)).or(Some(true)) ) == Some(true);

    return has_good_ls || has_good_rs;
}

fn find_splitters( input: &Vec<String> ) -> HashMap<(u64, u64), bool> { 
    let mut splitters = HashMap::<(u64, u64), bool>::new();
    let mut opener = (0,0);
    for (i, line) in input.iter().enumerate() { 
        for (j, c) in line.chars().enumerate() { 
            let location = (i as u64,j as u64);
            if c == 'S' { opener = location; }
            else if c == '^' && splits_beam(location, opener, &splitters) {
                splitters.insert(location, true);
            }
        }
    }

    println!("Found {} splitters.", splitters.len());
    splitters
}

pub fn solve(test: bool) -> Result<(), String> { 
    let input = utils::read_lines(test, "dec07")?;
    let splitters = find_splitters(&input);
    let mut n_beam_paths = HashMap::<u64, HashMap::<u64, u64>>::new();
    for (i, line) in input.iter().enumerate() { 
        for (j, _c) in line.chars().enumerate() { 
            let location = (i as u64, j as u64);
            if splitters.contains_key(&location) { 
                //println!("Splitter at: {location:?}");
                let n_paths_to_splitter = *n_beam_paths.entry(location.0)
                    .or_insert_with(HashMap::<u64, u64>::new)
                    .entry(location.1).or_insert(1);
                for d in [(j-1),(j+1)] {
                    for k in (i+1)..input.len() { 
                        let b = k as u64;
                        let h = n_beam_paths.entry(b)
                            .or_insert_with(HashMap::<u64, u64>::new);
                        let c = h.entry(d as u64)
                            .or_insert(0);
                        *c+=n_paths_to_splitter;
                        //println!(".. ({},{})-{}", b, d, *c);
                        if splitters.contains_key(&(b,d as u64)) { break; }
                    }
                }
                
            }
        }
    }

    let m = n_beam_paths.keys().max()
        .and_then(|n| n_beam_paths.get(n))
        .map(|m| m.values().sum::<u64>())
        .unwrap_or(0);

    println!("Found {m:?} paths.");

    Ok(())
}

#[cfg(test)]

mod tests {
    use std::collections::HashMap;
 
    
    #[test]
    fn test_basics() { 
        let l = Some(22);
        let m = None;
        assert_eq!( l.and_then(|i| m.and_then(|j| Some(i>j)).or_else(|| Some(true)) ) 
            == Some(true), true );
        assert_eq!(Some(false).or(Some(true)), Some(false));
    }

    #[test]
    fn test_hashmap() { 
        let mut h = HashMap::<u64, u64>::new();
        let i = 22 as u64;
        h.entry(i).and_modify(|c| *c += 1).or_insert(0);
        assert_eq!(h[&i], 0);
    }
}