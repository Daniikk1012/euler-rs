use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const MAX: u128 = 1_000_000_000_000;

    let mut a = 1;
    let mut b = 3;

    while b * (b - 1) * 2 < MAX * (MAX - 1) {
        (a, b) = (b, b * 6 - a - 2);
    }

    let result: u128 = b;

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
