use std::time::SystemTime;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        (a, b) = (b, a);
    }

    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn main() {
    let time = SystemTime::now();

    let result = (1..=20).fold(1, lcm);

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
