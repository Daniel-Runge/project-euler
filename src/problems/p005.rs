/// Problem 5
/// The smallest positive number that is evenly divisible by all of the numbers from 1 to 20
/// https://projecteuler.net/problem=5

pub fn solve() -> usize {
    let mut number = 1;
    for i in 11..=20 {
        println!("Number: {}, i: {}", number, i);
        number = lcm(number, i);
    }
    number
}

fn gcd(mut n:usize, mut m:usize) -> usize {
    if n == 0 { return m; }
    if m == 0 { return n; }
    let shift = (n | m).trailing_zeros();

    n >>= shift;
    m >>= shift;
    n >>= n.trailing_zeros();

    loop {
        m >>= m.trailing_zeros();
        if n > m {
            let temp: usize = n;
            n = m;
            m = temp;
        }
        m -= n;
        if m == 0 { 
            break; 
        }
    }
    n << shift
}

fn lcm(n:usize, m:usize) -> usize {
    n * m / gcd(n, m)
}

// 1 divides all integers
// 2 divides all even integers
// if 20 divides it, 10, 5 and 4 does too
// 19 is prime
// if 18, then 9, 6 and 3
// 17 is primes
// 16 then 8 and 4
// 15 then 5 and 3
// 14 then 7 
// 13 is prime
// 12 then 6 and 3
// 11 is prime




// if 10 divides it, 5 does too
// if 9 divides it, 3 does too
// if 8 divides it, 4 does too
// 7 is the largest prime number
// if 6 divides it, 3 does too