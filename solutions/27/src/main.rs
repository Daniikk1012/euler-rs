use std::time::SystemTime;

use primter::Primes;

fn main() {
    let time = SystemTime::now();

    const MAX: i32 = 999;

    let mut primes = Primes::new();

    let (a, b) = (-MAX..=MAX)
        .flat_map(|a| (-MAX..=MAX).map(move |b| (a, b)))
        .max_by_key(|(a, b)| {
            (0..)
                .take_while(|n| {
                    let number = n * n + a * n + b;
                    number > 0 && primes.is_prime_mut(number as usize)
                })
                .count()
        })
        .unwrap();

    let result = a * b;

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
