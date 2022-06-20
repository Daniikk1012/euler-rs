use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result: u32 = (1..1_000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
