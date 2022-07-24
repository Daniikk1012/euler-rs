use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut sum = [0; 10];
    let mut product = [0; 10];

    for base in 1..=1_000 {
        for digit in &mut product {
            *digit = 0;
        }

        product[0] = 1;

        for _ in 0..base {
            let mut remainder = 0;

            for digit in &mut product {
                *digit *= base;
                *digit += remainder;
                remainder = *digit / 10;
                *digit %= 10;
            }
        }

        let mut remainder = 0;

        for (sum_digit, &product_digit) in sum.iter_mut().zip(&product) {
            *sum_digit += product_digit;
            *sum_digit += remainder;
            remainder = *sum_digit / 10;
            *sum_digit %= 10;
        }
    }

    let result: String = sum
        .into_iter()
        .rev()
        .map(|digit| char::from_digit(digit, 10).unwrap())
        .collect();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
