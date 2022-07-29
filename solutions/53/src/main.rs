use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result: u64 = (1..=100)
        .map(|n| {
            (0..=n / 2)
                .skip_while(|&r| {
                    ((n - r + 1)..=n).product::<u64>()
                        / (1..=r).product::<u64>()
                        <= 1_000_000
                })
                .map(|r| if n % 2 == 0 && r == n / 2 { 1 } else { 2 })
                .sum::<u64>()
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
