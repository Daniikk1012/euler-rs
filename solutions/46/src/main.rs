use std::time::SystemTime;

use primter::Primes;

fn main() {
    let time = SystemTime::now();

    let mut primes = Primes::new();

    let result = (9..)
        .step_by(2)
        .find(|&number| {
            !primes.is_prime_mut(number)
                && (1..)
                    .map(|n| 2 * n * n)
                    .take_while(|&term| term < number)
                    .all(|term| {
                        primes
                            .iter()
                            .take_while(|&prime| prime + term <= number)
                            .all(|prime| prime + term != number)
                    })
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
