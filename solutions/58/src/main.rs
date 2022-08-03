use std::time::SystemTime;

use primter::Primes;

fn main() {
    let time = SystemTime::now();

    let mut primes = Primes::new();
    primes.generate_to(30_000);

    let mut all_count = 1;
    let mut prime_count = 0;

    let result = (3..)
        .step_by(2)
        .find(|&len| {
            all_count += 4;

            prime_count += (len * len - (len - 1) * 3..)
                .step_by(len - 1)
                .take(3)
                .filter(|&number| primes.is_prime(number))
                .count();

            prime_count * 10 < all_count
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
