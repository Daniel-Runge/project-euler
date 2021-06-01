/// Problem 3
/// The largest prime factor of the number 600851475143.
/// https://projecteuler.net/problem=3

pub fn solve() -> usize {
    let mut number = 600851475143;
    let mut factor = 0;
    for i in primes() {
        if number % i == 0 {
            factor = i;
        }
        while number % i == 0 && number >= 2 {
            number /= i;
        }
        if number == 1 {
            return factor;
        }
    }
    factor
}

struct Prime {
    current: usize,
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut current: usize = self.current + 1;
        while !is_prime(current) {
            current += 1;
        }
        self.current = current;
        Some(self.current)
    }
}

fn is_prime(prime: usize) -> bool {
    for i in 2..(prime /2 +1) {
        if prime % i == 0 {
            return false;
        }
    }
    true
}

fn primes() -> Prime {
    Prime { current: 1 }
}

