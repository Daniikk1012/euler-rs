use std::time::SystemTime;

use primal::Sieve;

const fn digits(mut n: usize) -> usize {
    let mut result = 0;

    while n > 0 {
        n /= 10;
        result += 1;
    }

    result
}

trait DigitSort {
    fn digit_sort(&mut self);
}

impl DigitSort for usize {
    fn digit_sort(&mut self) {
        let mut array = [0; digits(Self::MAX)];
        let mut mask = 1;

        for digit in &mut array {
            *digit = *self / mask % 10;
            mask = mask.wrapping_mul(10);
        }

        array.sort_unstable();

        *self = array.into_iter().fold(0, |result, digit| result * 10 + digit);
    }
}

fn main() {
    let time = SystemTime::now();

    const MAX: usize = 10_000_000;

    let sieve = Sieve::new((MAX as f32).sqrt() as usize + 1);

    let result = (2..MAX)
        .filter_map(|number| {
            let mut number_mut = number;
            number_mut.digit_sort();

            let totient: usize = sieve
                .factor(number)
                .unwrap()
                .into_iter()
                .map(|(prime, power)| prime.pow(power as u32 - 1) * (prime - 1))
                .product();
            let mut totient_mut = totient;
            totient_mut.digit_sort();

            if number_mut == totient_mut {
                Some((number, number as f32 / totient as f32))
            } else {
                None
            }
        })
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(number, _)| number)
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
