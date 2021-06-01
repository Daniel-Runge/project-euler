/// Problem 4
/// The largest palindrome made from the product of two 3-digit numbers
/// https://projecteuler.net/problem=4

pub fn solve() -> usize {
    for n in (100..=999).rev() {
        for m in (n-n/10..=n).rev() {
            if palindrome(n*m){
                return n*m;
            }
        }
        
    }
    0
}

fn palindrome(mut number: usize) -> bool {
    let original = number;
    let mut reverse = 0;
    let mut digit;
    while number > 0 {
        digit = number % 10;
        reverse = reverse * 10 + digit;
        number = number / 10;
    }
    original == reverse
}