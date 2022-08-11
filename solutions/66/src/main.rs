use std::{iter, time::SystemTime};

use num::{bigint::ToBigInt, Integer};

fn main() {
    let time = SystemTime::now();

    let result = (1..=1_000)
        .filter(|&d| ((d as f64).sqrt() as i128).pow(2) != d)
        .max_by_key(|&d| {
            let sqrt = (d as f64).sqrt();

            (1..)
                .find_map(|iterations| {
                    let mut vec = Vec::new();
                    let (mut left, mut right, mut bottom) =
                        (1, -sqrt as i128, 1);

                    loop {
                        let limit = (bottom as f64
                            / (left as f64 * sqrt + right as f64))
                            as i128;
                        (left, right, bottom) =
                            (left * bottom, -right * bottom, d - right.pow(2));
                        right -= limit * bottom;

                        vec.push(limit);

                        let gcd = left.gcd(&right).gcd(&bottom);
                        left /= gcd;
                        right /= gcd;
                        bottom /= gcd;

                        if bottom == 1 {
                            break;
                        }
                    }

                    let (mut numer, mut denom) =
                        (0.to_bigint().unwrap(), 1.to_bigint().unwrap());

                    for number in iter::once(sqrt as i128)
                        .chain(vec.into_iter().cycle())
                        .take(iterations)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                    {
                        numer += number * &denom;
                        (numer, denom) = (denom, numer);
                    }

                    if denom.pow(2) - d * numer.pow(2) == 1.to_bigint().unwrap()
                    {
                        Some(denom)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
