    use crate::utils;

fn get_joltage(config: &str, num_digits: usize) -> u64 { 
    let digits: Option<Vec<u32>> = config.chars()
        .map(|c| c.to_digit(10))
        .collect();
    let digits = digits.unwrap_or_else(|| { 
        eprintln!("Error parsing config: {}", config);
        Vec::new()
    });
    let mut joltage: u64 = 0;
    let mut idx = 0;
    for n in (0..num_digits).rev() { 
        let max_digit = digits[idx..digits.len()-n].iter().max().unwrap_or_else(|| &0);
        idx = digits[idx..digits.len()-n].iter()
            .position(|v| v == max_digit)
            .unwrap_or_else(|| 0) + 1 + idx;
        joltage += (*max_digit as u64)*10u64.pow(n as u32);
    };
    joltage
}

pub fn solve(test: bool) -> Result<(), String> { 
    let filename = if test { "dec03.test.input" } else { "dec03.input" };
    let configs = utils::read_lines(filename)?;
    let mut total_joltage = 0;
    for config in configs { 
        total_joltage += get_joltage(&config, 12);
    }
    println!("Sum of overall joltages: {}", total_joltage);
    Ok(())
}

#[cfg(test)]

mod tests { 
    use super::*;

    #[test]
    fn test_get_joltage() { 
        assert_eq!(get_joltage("9188182", 2), 98);
        assert_eq!(get_joltage("987654321111111", 12), 987654321111);
        assert_eq!(get_joltage("111111119", 2), 19);
        assert_eq!(get_joltage("111111119", 3), 119);
    }
}