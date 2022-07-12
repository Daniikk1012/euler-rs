use std::time::SystemTime;

fn is_palindrome(number: u32, base: u32) -> bool {
    let mut left = 1;

    while left <= number {
        left *= base;
    }

    left /= base;

    let mut right = 1;

    while left > right {
        if number / left % base != number / right % base {
            return false;
        }

        left /= base;
        right *= base;
    }

    true
}

fn main() {
    let time = SystemTime::now();

    let result: u32 = (1..1_000_000)
        .filter(|&number| is_palindrome(number, 10) && is_palindrome(number, 2))
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
