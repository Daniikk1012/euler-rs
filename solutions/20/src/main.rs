use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut number = vec![1];

    for factor in 1..=100 {
        let mut remainder = 0;

        for digit in &mut number {
            *digit *= factor;
            *digit += remainder;
            remainder = *digit / 10;
            *digit %= 10;
        }

        while remainder > 0 {
            number.push(remainder % 10);
            remainder /= 10;
        }
    }

    let result: u32 = number.into_iter().sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
