use crate::utils;

fn is_invalid(s: &str) -> bool { 
    for i in 1..(s.len()/2+1) { 
        let sl1 = &s[0..i];
        let nr = s.len()/i;
        if sl1.repeat(nr) == s { 
            return true;
        }
    };
    false
}

fn get_invalids(start: i64, end: i64) -> Vec<i64> { 
    let mut result: Vec<i64> = Vec::new();
    for i in start..end+1 { 
        let s = i.to_string();
        //let l = s.len();
        //if s[0..l/2] == s[l/2..l] { 
        if is_invalid(&s) {
            result.push(i) 
        }
    }
    result
}

pub fn solve(test: bool) -> Result<(), String> { 
    let mut total: i64 = 0;
    let ranges= utils::read_lines(test, "dec02")?
        .first()
        .expect("failed to read file")
        .split(',').map(|s| s.to_owned()).collect::<Vec<String>>();
    for s in ranges { 
        let nums = s.split('-')
            .map(|i| i.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>();
        let ( start, end ) = match nums { 
            Ok(n) => (n[0], n[1]),
            Err(_e) => (0,0)
        };
        for x in get_invalids(start, end) { 
            total += x
        }
    }
    println!("Total was: {}", total);
    Ok(())
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_repeat_nums() {
        assert_eq!(get_invalids(11,33), vec![11,22,33]);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(is_invalid("111"), true);
        assert_eq!(is_invalid("232323"), true);
        assert_eq!(is_invalid("446446"), true);
        assert_eq!(is_invalid("123123"), true);        
        assert_eq!(is_invalid("131"), false);
    }    
}