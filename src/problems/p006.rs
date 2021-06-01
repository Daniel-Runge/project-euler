/// Problem 6
/// The difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
/// https://projecteuler.net/problem=6

pub fn solve() -> usize {
    let mut sum_of_squares = 0;
    let mut sum = 0;
    for i in 0..=100 {
        sum_of_squares += i*i;
        sum += i;
    }
    sum * sum - sum_of_squares
}