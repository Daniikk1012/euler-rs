use std::time::SystemTime;

use primter::Primes;

fn main() {
    let time = SystemTime::now();

    const MAX: usize = 1_000_000;

    let mut primes = Primes::new();
    primes.generate_to(MAX);

    let max_len = (1..)
        .find(|&len| primes.primes()[..len].iter().sum::<usize>() > MAX)
        .unwrap();

    let result = (1..max_len)
        .rev()
        .find_map(|len| {
            primes
                .primes()
                .windows(len)
                .map(|window| window.iter().sum())
                .take_while(|&sum| sum < MAX)
                .find(|&sum| primes.is_prime(sum))
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
