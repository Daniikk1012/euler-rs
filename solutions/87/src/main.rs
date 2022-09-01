use std::{collections::HashSet, time::SystemTime};

use primal::Sieve;

fn main() {
    let time = SystemTime::now();

    const MAX: usize = 50_000_000;

    let mut set = HashSet::new();
    let sieve = Sieve::new(((MAX as f32).sqrt() * 1.1) as usize);

    for number in (1..)
        .map(|index| sieve.nth_prime(index).pow(2))
        .take_while(|&number| number < MAX)
        .flat_map(|number| {
            let sieve = &sieve;
            (1..)
                .map(move |index| number + sieve.nth_prime(index).pow(3))
                .take_while(|&number| number < MAX)
        })
        .flat_map(|number| {
            let sieve = &sieve;
            (1..)
                .map(move |index| number + sieve.nth_prime(index).pow(4))
                .take_while(|&number| number < MAX)
        })
    {
        set.insert(number);
    }

    let result = set.len();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
