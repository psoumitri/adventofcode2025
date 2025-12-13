use std::env;
use crate::utils;

fn rotate_dial( start_pos: i64, rotation: &str ) -> Result<(i64, i64), String> { 
    let mut chars = rotation.chars();
    let direction = chars.next().ok_or("no direction")?;
    let mut distance = chars.as_str()
        .parse::<i64>()
        .map_err(|_| "failed to get distance")?;
    let dial_size = 100;
    let mut nz = distance.div_euclid(dial_size);
    distance = distance.rem_euclid(dial_size);
    let d = match direction { 
        'L' => start_pos - distance,
        'R' => start_pos + distance,
        _ => start_pos
    };
    if d > 99 || ( d <= 0 && start_pos != 0 ) { nz += 1 }
    let end_pos = d.rem_euclid(dial_size);
    Ok( (nz, end_pos) )
}

fn determine_code( filename: &str ) -> Result<i64, String> { 
    let instr = utils::read_lines(filename)?;
    let mut start_pos = 50;
    let mut code = 0;
    for rotation in instr { 
        let (nz, sp) = rotate_dial(start_pos, &rotation)?;
        //if sp == 0 { code += 1 }
        code += nz;
        start_pos = sp;
    };
    Ok(code)
}

pub fn solve(test: bool) -> Result<(), String> { 
    let filename = if test { "dec01.test.input" } else { "dec01.input" };
    let s = determine_code(filename);
    match s { 
        Ok(c) => println!("Code is: {}", c),
        Err(e) => eprintln!("Failed with: {}", e)
    };
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn rotate_dial_tests() { 
        assert_eq!(rotate_dial(65,"L12"), Ok((0,53)));
        assert_eq!(rotate_dial(65,"L112"), Ok((1,53)));
        assert_eq!(rotate_dial(65,"R35"), Ok((1,0)));
        assert_eq!(rotate_dial(65,"L1012"), Ok((10,53)));
        assert_eq!(rotate_dial(65,"L10012"), Ok((100,53)));
        
        assert_eq!(rotate_dial(1,"L1"), Ok((1,0)));
        assert_eq!(rotate_dial(1,"L2"), Ok((1,99)));
        assert_eq!(rotate_dial(0,"L1"), Ok((0,99)));
        assert_eq!(rotate_dial(0,"R100"), Ok((1,0)));
        assert_eq!(rotate_dial(0,"R2000"), Ok((20,0)));
        assert_eq!(rotate_dial(0,"L100"), Ok((1,0)));
        assert_eq!(rotate_dial(0,"L2000"), Ok((20,0)));
        assert_eq!(rotate_dial(1,"L100"), Ok((1,1)));

        assert_eq!(rotate_dial(0,"R1"), Ok((0,1)));
        assert_eq!(rotate_dial(61,"R39"), Ok((1,0)));
        assert_eq!(rotate_dial(61,"R40"), Ok((1,1)));
        assert_eq!(rotate_dial(61,"L61"), Ok((1,0))); // generalize this. 
        
        assert_eq!(rotate_dial(99,"R1"), Ok((1,0)));
        assert_eq!(rotate_dial(99,"R100"), Ok((1,99)));
        assert_eq!(rotate_dial(99,"L99"), Ok((1,0)));
        assert_eq!(rotate_dial(99,"L199"), Ok((2,0)));
    }
}