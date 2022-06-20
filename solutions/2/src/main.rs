use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut result: u64 = 0;

    let mut a = 0;
    let mut b = 1;

    while b < 4_000_000 {
        (a, b) = (b, a + b);

        if b % 2 == 0 {
            result += b;
        }
    }

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
