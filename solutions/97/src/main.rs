use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut array = [0; 10];
    array[0] = 1;

    for _ in 0..7_830_457 {
        let mut remainder = 0;

        for digit in &mut array {
            *digit *= 2;
            *digit += remainder;
            remainder = *digit / 10;
            *digit %= 10;
        }
    }

    let mut remainder = 0;

    for digit in &mut array {
        *digit *= 28_433;
        *digit += remainder;
        remainder = *digit / 10;
        *digit %= 10;
    }

    let mut remainder = 0;
    array[0] += 1;

    for digit in &mut array {
        *digit += remainder;
        remainder = *digit / 10;
        *digit %= 10;
    }

    let result: String = array
        .into_iter()
        .rev()
        .map(|digit| char::from_digit(digit, 10).unwrap())
        .collect();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
