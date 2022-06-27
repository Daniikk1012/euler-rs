use std::{collections::HashMap, time::SystemTime};

fn d(number: u32) -> u32 {
    (2..)
        .take_while(|divisor| divisor * divisor <= number)
        .map(|divisor| {
            if divisor * divisor == number {
                divisor
            } else if number % divisor == 0 {
                divisor + number / divisor
            } else {
                0
            }
        })
        .sum::<u32>()
        + 1
}

fn main() {
    let time = SystemTime::now();

    let mut cache = HashMap::new();

    let result: u32 = (1..10_000)
        .filter(|number| {
            let pair = d(*number);
            cache.insert(*number, pair);

            let pair_pair = *cache.entry(pair).or_insert_with(|| d(pair));

            *number == pair_pair && *number != pair
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
