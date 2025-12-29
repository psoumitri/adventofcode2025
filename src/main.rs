mod dec11;
mod utils;

fn main() { 
    println!("Welcome to my advent of code programs 2025!");
    dec11::solve(false)
        .unwrap_or_else(|e| eprintln!("{}",e));
}