use std::time::SystemTime;

use num::{
    bigint::ToBigInt, BigInt, BigRational, FromPrimitive, One, ToPrimitive,
};

fn big_sqrt(number: u32) -> BigRational {
    let number = BigRational::from_u32(number).unwrap();
    let two = BigRational::from_u32(2).unwrap();
    let threshold =
        BigRational::new(BigInt::one(), 10.to_bigint().unwrap().pow(200));
    let mut approx = number.clone();

    while (approx.pow(2) - &number) > threshold {
        approx = (&approx + (&number / &approx)) / &two;
    }

    approx
}

fn main() {
    let time = SystemTime::now();

    let ten = BigRational::from_u32(10).unwrap();

    let result: u32 = (1..=100)
        .filter(|&number| ((number as f32).sqrt() as u32).pow(2) != number)
        .map(big_sqrt)
        .map(|mut sqrt| {
            let mut result = 0;

            let whole = sqrt.floor().to_u32().unwrap();
            let mut mask = 1;

            while mask <= whole {
                mask *= 10;
            }

            mask /= 10;
            let mut mask = BigRational::from_u32(mask).unwrap();

            for _ in 0..100 {
                let digit = (&sqrt / &mask).floor();
                result += digit.to_u32().unwrap();
                sqrt -= digit * &mask;
                mask /= &ten;
            }

            result
        })
        .sum();

    println!();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
