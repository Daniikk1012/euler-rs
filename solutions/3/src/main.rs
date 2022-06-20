use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut result = 0;

    let mut number: u64 = 600851475143;

    for factor in 2.. {
        if factor * factor > number {
            break;
        }

        while number % factor == 0 {
            number /= factor;
            result = factor;
        }
    }

    if number > 1 {
        result = number;
    }

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
