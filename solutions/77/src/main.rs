use std::time::SystemTime;

use primal::Sieve;

fn calculate(sieve: &Sieve, left: usize, max: usize) -> usize {
    match left {
        0 => 1,
        1 => 0,
        _ => (1..)
            .map(|index| sieve.nth_prime(index))
            .take_while(|&prime| prime <= left.min(max))
            .map(|prime| calculate(sieve, left - prime, prime))
            .sum(),
    }
}

fn main() {
    let time = SystemTime::now();

    let sieve = Sieve::new(1_000_000);

    let result = (1..)
        .find(|&number| calculate(&sieve, number, number) > 5_000)
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
