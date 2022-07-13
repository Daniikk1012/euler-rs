use std::time::SystemTime;

use primter::Primes;

fn is_truncatable_prime(number: usize, primes: &mut Primes) -> bool {
    let mut mask = 1;

    while mask <= number {
        mask *= 10;
    }

    while mask > 1 {
        if !primes.is_prime_mut(number % mask)
            || mask <= number && !primes.is_prime_mut(number / mask)
        {
            return false;
        }

        mask /= 10;
    }

    true
}

fn main() {
    let time = SystemTime::now();

    let mut primes = Primes::new();

    let result: usize = (23..)
        .filter(|&number| is_truncatable_prime(number, &mut primes))
        .take(11)
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
