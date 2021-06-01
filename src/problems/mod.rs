pub mod p001;
pub mod p002;
pub mod p003;
pub mod p004;

pub const SOLVED: usize = 4;
pub const PROBLEMS: [fn() -> usize; SOLVED] = [
    p001::solve,
    p002::solve,
    p003::solve,
    p004::solve,
    ];

pub fn solve(problem: usize) -> usize{
    PROBLEMS[problem-1]()
}
