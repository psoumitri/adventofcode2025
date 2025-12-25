mod dec10;
mod utils;

fn main() { 
    println!("Welcome to my advent of code programs 2025!");
    dec10::solve(false)
        .unwrap_or_else(|e| eprintln!("{}",e));
}