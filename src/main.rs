use std::env;
mod problems;

fn main() {
    for argument in env::args().skip(1) {
        match argument.parse::<usize>() {
            Ok(problem) => solve(problem),
            Err(e) => println!("Error line {}", e)
        }
    }
}

fn solve(problem: usize){
    match problem {
        1..=problems::SOLVED => println!("Problem {} has the following solution: {}", problem, problems::solve(problem)),
        _ => println!("Problem {} has no solution yet :(", problem),
    };
}