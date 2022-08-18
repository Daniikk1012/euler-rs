use std::time::SystemTime;

use num_integer::Integer;

fn main() {
    let time = SystemTime::now();

    let result: usize = (1..=12_000)
        .map(|denom| {
            (denom / 3 + 1..=(denom - 1) / 2)
                .filter(|&numer| numer.gcd(&denom) == 1)
                .count()
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
