use std::time::SystemTime;

fn sum_divisible(possible: &mut [bool; 10], number: u64) -> u64 {
    match number {
        0 if !possible[0] => 0,
        1_000..=9_999 if number % 1_000 % 2 != 0 => 0,
        10_000..=99_999 if number % 1_000 % 3 != 0 => 0,
        100_000..=999_999 if number % 1_000 % 5 != 0 => 0,
        1_000_000..=9_999_999 if number % 1_000 % 7 != 0 => 0,
        10_000_000..=99_999_999 if number % 1_000 % 11 != 0 => 0,
        100_000_000..=999_999_999 if number % 1_000 % 13 != 0 => 0,
        1_000_000_000..=9_999_999_999 => {
            if number % 1_000 % 17 != 0 {
                0
            } else {
                number
            }
        }
        _ => (0..10)
            .filter_map(|digit| {
                if possible[digit] {
                    possible[digit] = false;
                    let result =
                        sum_divisible(possible, number * 10 + digit as u64);
                    possible[digit] = true;
                    Some(result)
                } else {
                    None
                }
            })
            .sum(),
    }
}

fn main() {
    let time = SystemTime::now();

    let result = sum_divisible(&mut [true; 10], 0);

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
