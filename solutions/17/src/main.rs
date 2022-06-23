use std::time::SystemTime;

fn to_word(number: u32) -> u32 {
    match number {
        1 | 2 | 6 | 10 => 3,
        4 | 5 | 9 => 4,
        3 | 7 | 8 | 40 | 50 | 60 => 5,
        11 | 12 | 20 | 30 | 80 | 90 => 6,
        15 | 16 | 70 => 7,
        13 | 14 | 18 | 19 => 8,
        17 => 9,
        20..=99 => {
            to_word(number - number % 10)
                + if number % 10 > 0 { to_word(number % 10) } else { 0 }
        }
        100..=999 => {
            to_word(number / 100)
                + 7
                + if number % 100 > 0 { 3 + to_word(number % 100) } else { 0 }
        }
        1_000 => 11,
        _ => unreachable!(),
    }
}

fn main() {
    let time = SystemTime::now();

    let result: u32 = (1..=1_000).map(to_word).sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
