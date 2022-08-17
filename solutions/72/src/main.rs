use std::time::SystemTime;

use primal::Sieve;

fn main() {
    let time = SystemTime::now();

    const MAX: usize = 1_000_000;

    let sieve = Sieve::new((MAX as f32).sqrt() as usize + 1);

    let result: usize = (2..=MAX)
        .map(|denom| {
            sieve
                .factor(denom)
                .unwrap()
                .into_iter()
                .map(|(prime, power)| prime.pow(power as u32 - 1) * (prime - 1))
                .product::<usize>()
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
