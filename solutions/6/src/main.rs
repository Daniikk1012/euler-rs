use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const MAX: u64 = 100;

    let result =
        (MAX * (MAX + 1) / 2).pow(2) - MAX * (MAX + 1) * (MAX * 2 + 1) / 6;

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
