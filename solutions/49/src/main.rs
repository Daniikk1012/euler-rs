use std::time::SystemTime;

use primter::Primes;

fn number_to_ordered(number: usize) -> Vec<u8> {
    let mut ordered = number.to_string().into_bytes();

    for digit in &mut ordered {
        *digit = (*digit as char).to_digit(10).unwrap() as u8;
    }

    ordered.sort_unstable();

    ordered
}

fn main() {
    let time = SystemTime::now();

    const MIN: usize = 1_000;
    const MAX: usize = 10_000;

    let mut primes = Primes::new();
    primes.generate_to(MAX);

    let result = primes
        .primes()
        .iter()
        .map(ToOwned::to_owned)
        .skip_while(|&prime| prime < MIN)
        .take_while(|&prime| prime < MAX)
        .find_map(|first| {
            if first == 1_487 {
                return None;
            }

            let first_ordered = number_to_ordered(first);

            primes
                .primes()
                .iter()
                .map(ToOwned::to_owned)
                .skip_while(|&prime| prime <= first)
                .take_while(|&prime| prime < MAX)
                .find_map(|second| {
                    let third = second + second - first;

                    if third < MAX
                        && primes.is_prime(third)
                        && first_ordered == number_to_ordered(second)
                        && first_ordered == number_to_ordered(third)
                    {
                        Some(first * MAX * MAX + second * MAX + third)
                    } else {
                        None
                    }
                })
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
