/// Problem 2
/// The sum of the even-valued Fibonacci terms.
/// https://projecteuler.net/problem=2

pub fn solve() -> usize {
    let mut sum = 0;
    for i in fibonacci().take_while(|&x| x < 4000000).filter(|&x| x & 1 == 0){
        sum += i;
    }
    sum
}

struct Fibonacci {
    previous: usize,
    current: usize,
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let next = self.current + self.previous;

        self.previous = self.current;
        self.current = next;

        Some(self.current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { current: 1, previous: 1 }
}