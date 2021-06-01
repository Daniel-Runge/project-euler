/// Problem 1
/// The sum of all the multiples of 3 or 5 below 1000.
/// https://projecteuler.net/problem=1

pub fn solve() -> usize{
    let sum5 = (1..200).fold(0, |total, next| total + 3*next + 5*next);
    let sum3 = (200..=333).fold(0, |total, next| total + 3*next);
    let sum15 = (1..67).fold(0, |total, next| total + 15*next);
    sum5 + sum3 - sum15
}