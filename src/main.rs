//mod dec01;
//mod dec02;
//mod dec03;
//mod dec04;
mod dec05;
mod utils;

fn main() { 
    println!("Welcome to my advent of code programs 2025!");
    //dec01::solve(false)
    //dec02::solve(false)
    //dec03::solve(false)
    //dec04::solve(false)
    dec05::solve(false)
        .unwrap_or_else(|e| eprintln!("{}",e));
}