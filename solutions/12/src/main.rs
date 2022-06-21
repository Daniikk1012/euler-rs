use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result: u32 = (1..)
        .map(|n| n * (n + 1) / 2)
        .find(|n| {
            (1..)
                .take_while(|d| d * d <= *n)
                .map(|d| {
                    if d * d == *n {
                        1
                    } else if n % d == 0 {
                        2
                    } else {
                        0
                    }
                })
                .sum::<u32>()
                > 500
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
