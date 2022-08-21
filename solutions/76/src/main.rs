use std::time::SystemTime;

fn calculate(left: u32, max: u32) -> usize {
    if left == 0 {
        1
    } else {
        (1..=left.min(max)).map(|next| calculate(left - next, next)).sum()
    }
}

fn main() {
    let time = SystemTime::now();

    let result = calculate(100, 100) - 1;

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
