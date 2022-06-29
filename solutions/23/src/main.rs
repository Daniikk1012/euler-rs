use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const MAX: u32 = 28_123;

    let abundants: Vec<_> = (1..=MAX)
        .filter(|number| {
            (2..)
                .take_while(|divisor| divisor * divisor <= *number)
                .filter(|divisor| number % divisor == 0)
                .map(|divisor| {
                    if divisor * divisor == *number {
                        divisor
                    } else {
                        divisor + number / divisor
                    }
                })
                .sum::<u32>()
                + 1
                > *number
        })
        .collect();

    let result: u32 = (1..=MAX)
        .filter(|number| {
            abundants
                .iter()
                .take_while(|left| *left * 2 <= *number)
                .find(|left| abundants.binary_search(&(number - *left)).is_ok())
                .is_none()
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
