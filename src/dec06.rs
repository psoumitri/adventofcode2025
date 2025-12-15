use crate::utils;

/*
fn solve_part1(operations: Vec<&str>, inputstr: Vec<String> ) {
    let mut results = Vec::<i64>::new();
    for data in inputstr { 
        let mut nums = data.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap_or_else(|_e| 0))
            .collect::<Vec<i64>>();
        if results.is_empty() { results.append(&mut nums); continue; }
        let results2 = results.clone();
        for (i, (&op, (&r, n))) in 
            operations.iter().zip((results2).iter().zip(nums)).enumerate() {
            if op == "*" { results[i] = r*n }
            else { results[i] = r+n }
        }
    }
    println!("[Part1] Overall total: {}", results.iter().sum::<i64>());
}
*/

fn operate( operation: &str, nums: &Vec<i64> ) -> i64 { 
    let result = if operation == "*" {
        nums.iter().copied().reduce(|a,b| a*b)
    } else {
        nums.iter().copied().reduce(|a,b| a+b)
    };
    match result { 
        Some(r) => r,
        None => { 
            eprintln!("failed to operate: {} over {:?}", operation, nums);
            0
        }
    }
}

fn solve_part2(operations: Vec<&str>, inputstr: Vec<String>) {
    let mut t_inputstr = Vec::<String>::new();
    let row_size = inputstr.first().map(|s| s.len()).unwrap_or_else(|| 0);
    for i in 0..row_size { 
        let c1 = (0..inputstr.len()).into_iter()
            .map(|j| inputstr[j].chars().nth(i)
                .unwrap_or_else(|| ' ').to_owned())
            .collect::<String>();
        t_inputstr.push(c1);
    }
    let mut result: i64 = 0;
    let mut i = 0;
    let mut nums = Vec::<i64>::new();
    for s in t_inputstr {
        //println!("{:?}", s);
        if s.trim().is_empty() { 
            //println!("Applying: {} on {:?}", operations[i], &nums);
            result += operate(operations[i], &nums);
            nums.clear();
            i += 1;
        } else { 
            let num = s.trim().parse::<i64>()
                .inspect_err(|e| eprintln!("Failed with: {:?}", e))
                .unwrap_or_else(|_e | 0);
            nums.push(num);
        }
    };
    result += operate(operations[i], &nums);
    println!("[Part2] Overall total: {}", result);
}

pub fn solve(test: bool) -> Result<(), String> { 
    let mut inputstr = utils::read_lines(test, "dec06")?;
    let operations = inputstr.pop()
        .unwrap_or_else(|| "".to_owned());
    let operations = operations
        .split_whitespace()
        .collect::<Vec<&str>>();
    //solve_part1(operations, inputstr);
    solve_part2(operations, inputstr);
    Ok(())
}


#[cfg(test)]

mod tests { 
    //use super::*;

    #[test]
    fn test_stringlen() { 
        let m = Some("mary    had a little lamb");
        assert_eq!(m.unwrap_or_else(|| "").len(), 10);
    }

}