use std::time::SystemTime;

use primter::Primes;

fn main() {
    let time = SystemTime::now();

    let mut primes = Primes::new();
    primes.generate_to(10_000_000);

    let result = (1..10_000_000)
        .rev()
        .filter(|&number| primes.is_prime(number))
        .find(|&prime| {
            let mut count = 0;
            let mut mask = 1;

            while mask <= prime {
                count += 1;
                mask *= 10;
            }

            let mut digits = [false; 10];
            digits[0] = true;

            for digit in &mut digits[count + 1..] {
                *digit = true;
            }

            mask /= 10;

            while mask > 0 {
                if digits[prime / mask % 10] {
                    return false;
                }

                digits[prime / mask % 10] = true;
                mask /= 10;
            }

            true
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
