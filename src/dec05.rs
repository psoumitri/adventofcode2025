use crate::utils;

fn get_is_fresh( ranges: &Vec<(u64, u64)>, num: u64 ) -> bool { 
    let mut in_range = false;
    for (start, end) in ranges { 
        if num >= *start && num <= *end { in_range = true; break }
    }
    in_range
}

pub fn solve(test: bool) -> Result<(), String> { 
    let input = utils::read_lines(test, "dec05")?;
    let mut ranges = Vec::<(u64, u64)>::new();
    let as_u64 = |s: &str| {s.parse::<u64>()
        .inspect_err(|e| eprintln!("failed to parse to u64: {}", e))
        .unwrap_or_else(|_e| 0)};
    let mut input_started = false;
    let mut fresh_count = 0;
    for line in input { 
        if line.trim().is_empty() { input_started = true }
        if input_started { 
            if !line.trim().is_empty() && get_is_fresh( &ranges, as_u64(line.trim()) ) { 
                fresh_count += 1 
            }
        } else { 
            // we build the ranges 
            let (start, end) = line.split_once('-')
                .unwrap_or_else(|| ("",""));
            ranges.push((as_u64(start), as_u64(end)));
        }
    }
    println!("Found {} fresh veggies", fresh_count);
    let mut ranges2 = Vec::<(u64, u64)>::new();
    ranges.sort();
    for (s,e) in ranges { 
        if ranges2.is_empty() { ranges2.push((s,e)); continue }
        let &(s1, e1) = ranges2.last().unwrap();
        if e1<s { ranges2.push((s,e)) }
        else { 
            ranges2.pop();
            ranges2.push((s1,e.max(e1)))
        }
    };
    let n_fresh_ingr = ranges2.iter().map(|(s,e)| e-s+1).sum::<u64>();
    println!("Found {} IDs referencing fresh ingredients", n_fresh_ingr);
    Ok(())
}