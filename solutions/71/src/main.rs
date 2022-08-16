use std::time::SystemTime;

use num_integer::Integer;

fn main() {
    let time = SystemTime::now();

    let result = (1..=1_000_000)
        .map(|denom| {
            let numer = (denom * 3 - 1) / 7;
            (numer / numer.gcd(&denom), numer as f32 / denom as f32)
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(numer, _)| numer)
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
