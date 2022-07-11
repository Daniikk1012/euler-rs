use std::time::SystemTime;

use primter::Primes;

fn main() {
    let time = SystemTime::now();

    let mut primes = Primes::new();
    primes.generate_to(1_000_000);

    let result = (1..1_000_000)
        .filter(|&number| {
            let mut mask = 1;

            while mask * 10 <= number {
                mask *= 10;
            }

            let mut number_mut = number;

            while {
                number_mut = number_mut % mask * 10 + number_mut / mask % 10;

                if !primes.is_prime(number_mut) {
                    return false;
                }

                number_mut != number
            } {}

            true
        })
        .count();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
