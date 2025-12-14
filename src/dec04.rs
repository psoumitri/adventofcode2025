
use crate::utils; 

fn count_rolls(grid_slice: &Vec<&[char]>) -> usize { 
    grid_slice.iter()
        .flat_map(|s| s.iter())
        .filter(|&&c| c == '@')
        .count()
}

fn count_pickable_rolls(grid: &Vec<Vec<char>> ) -> ( Vec<Vec<char>>, u64 ) { 
    let mut grid_without_rolls = grid.clone();
    let mut c = 0;
    let max_rows = grid.len();
    for (row_pos, row) in grid.iter().enumerate() { 
        let max_cols = row.len();
        for (col_pos, thing) in row.iter().enumerate() {
            if *thing == '@' { 
                let top = row_pos.saturating_sub(1);
                let bottom = (row_pos+2).min(max_rows);
                let left = col_pos.saturating_sub(1);
                let right = (col_pos+2).min(max_cols);
                let neighborhood: &Vec<&[char]> = &grid[top..bottom].iter()
                    .map(|r| &r[left..right]).collect();
                let n_rolls_around = count_rolls(neighborhood);
                if n_rolls_around < 5 { 
                    c+=1;
                    grid_without_rolls[row_pos][col_pos] = '.'; 
                }
            }
        }
    }
    ( grid_without_rolls, c )
}

pub fn solve(test: bool) -> Result<(), String> { 
    let grid : Vec<Vec<char>> = utils::read_lines(test, "dec04")
        .unwrap_or_else(|_e| {Vec::new()})
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut have_rolls_to_pick = true;
    let mut total_rolls_picked = 0;
    let mut target_grid = grid;
    while have_rolls_to_pick { 
        let ( updated_grid, rolls_picked ) = count_pickable_rolls(&target_grid);
        total_rolls_picked += rolls_picked;
        if rolls_picked == 0 { have_rolls_to_pick = false }
        target_grid = updated_grid;
    }
    println!("Count total rolls picked: {}", total_rolls_picked);
    Ok(())
}
