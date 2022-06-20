use std::time::SystemTime;

use primter::Primes;

fn main() {
    let time = SystemTime::now();

    const MAX: usize = 2_000_000;

    let mut primes = Primes::new();
    primes.generate_to(MAX);

    let result: usize =
        primes.into_iter().take_while(|prime| *prime < MAX).sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
