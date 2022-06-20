use std::time::SystemTime;

use primter::Primes;

fn main() {
    let time = SystemTime::now();

    let mut primes = Primes::new();
    primes.generate_amount(10_001);

    let result = primes.primes()[10_000];

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
