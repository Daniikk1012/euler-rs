use std::{collections::HashMap, time::SystemTime};

use primter::Primes;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Family {
    len: u8,
    mask: u8,
    n: usize,
}

impl Family {
    fn new(len: u8, mask: u8, mut n: usize) -> Option<Self> {
        let mut digit = None;
        let mut mask_mut = mask;
        let mut result = 0;
        let mut power = 1;

        while n > 0 {
            if mask_mut & 1 == 1 {
                result += n % 10 * power;
            } else if let Some(digit) = digit {
                if n % 10 != digit {
                    return None;
                }
            } else {
                digit = Some(n % 10);
            }

            mask_mut >>= 1;
            power *= 10;
            n /= 10;
        }

        Some(Family { len, mask, n: result })
    }
}

fn digits(mut n: usize) -> u8 {
    let mut count = 0;

    while n > 0 {
        n /= 10;
        count += 1;
    }

    count
}

fn main() {
    let time = SystemTime::now();

    let mut primes = Primes::new();
    let mut families: HashMap<Family, (usize, usize)> = HashMap::new();

    for prime in primes.iter().take(100_000) {
        let len = digits(prime);

        for mask in 0..1 << len {
            if let Some(family) = Family::new(len, mask, prime) {
                families
                    .entry(family)
                    .and_modify(|(_, amount)| *amount += 1)
                    .or_insert((prime, 1));
            }
        }
    }

    let result =
        families.into_values().find(|(_, amount)| *amount == 8).unwrap().0;

    println!("Result: {}", result);

    match time.elapsed() {
        Ok(value) => println!("Elapsed time: {}ms", value.as_millis()),
        Err(error) => eprintln!("Error calculating time: {}", error),
    }
}
