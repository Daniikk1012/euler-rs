use std::{iter, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let (mut numer, mut denom) = (vec![0], vec![1]);
    let mut tmp = Vec::new();

    for number in iter::once(2)
        .chain((1..).flat_map(|k| [1, 2 * k, 1].into_iter()))
        .take(100)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        tmp.clone_from(&denom);

        let mut remainder = 0;

        for (&digit_numer, digit_denom) in
            numer.iter().chain(iter::repeat(&0)).zip(&mut denom)
        {
            *digit_denom *= number;
            *digit_denom += digit_numer;
            *digit_denom += remainder;
            remainder = *digit_denom / 10;
            *digit_denom %= 10;
        }

        while remainder > 0 {
            denom.push(remainder % 10);
            remainder /= 10;
        }

        numer.clone_from(&tmp);
    }

    let result: u32 = denom.into_iter().sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
