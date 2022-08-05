use std::time::SystemTime;

use primter::Primes;

fn concat(a: usize, b: usize) -> usize {
    let mut mask = 1;

    while mask <= b {
        mask *= 10;
    }

    a * mask + b
}

fn main() {
    let time = SystemTime::now();

    const MAX: usize = 10_000;

    let mut primes = Primes::new();
    primes.generate_to(MAX);

    let result: usize = primes
        .primes()
        .iter()
        .cloned()
        .take_while(|&prime| prime < MAX)
        .flat_map(|p1| {
            primes
                .primes()
                .iter()
                .cloned()
                .take_while(|&prime| prime < MAX)
                .map(move |p2| [p1, p2])
        })
        .filter(|ps| {
            primes.is_prime(concat(ps[0], ps[1]))
                && primes.is_prime(concat(ps[1], ps[0]))
        })
        .flat_map(|[p1, p2]| {
            primes
                .primes()
                .iter()
                .cloned()
                .take_while(|&prime| prime < MAX)
                .map(move |p3| [p1, p2, p3])
        })
        .filter(|ps| {
            primes.is_prime(concat(ps[0], ps[2]))
                && primes.is_prime(concat(ps[1], ps[2]))
                && primes.is_prime(concat(ps[2], ps[0]))
                && primes.is_prime(concat(ps[2], ps[1]))
        })
        .flat_map(|[p1, p2, p3]| {
            primes
                .primes()
                .iter()
                .cloned()
                .take_while(|&prime| prime < MAX)
                .map(move |p4| [p1, p2, p3, p4])
        })
        .filter(|ps| {
            primes.is_prime(concat(ps[0], ps[3]))
                && primes.is_prime(concat(ps[1], ps[3]))
                && primes.is_prime(concat(ps[2], ps[3]))
                && primes.is_prime(concat(ps[3], ps[0]))
                && primes.is_prime(concat(ps[3], ps[1]))
                && primes.is_prime(concat(ps[3], ps[2]))
        })
        .flat_map(|[p1, p2, p3, p4]| {
            primes
                .primes()
                .iter()
                .cloned()
                .take_while(|&prime| prime < MAX)
                .map(move |p5| [p1, p2, p3, p4, p5])
        })
        .find_map(|ps| {
            if primes.is_prime(concat(ps[0], ps[4]))
                && primes.is_prime(concat(ps[1], ps[4]))
                && primes.is_prime(concat(ps[2], ps[4]))
                && primes.is_prime(concat(ps[3], ps[4]))
                && primes.is_prime(concat(ps[4], ps[0]))
                && primes.is_prime(concat(ps[4], ps[1]))
                && primes.is_prime(concat(ps[4], ps[2]))
                && primes.is_prime(concat(ps[4], ps[3]))
            {
                Some(ps.into_iter().sum())
            } else {
                None
            }
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
