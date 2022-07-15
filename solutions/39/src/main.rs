use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result = (1..=1_000u32)
        .max_by_key(|&max| {
            (1..max)
                .filter(|&a| {
                    let b = if let Some(b) = (a..max)
                        .take_while(|b| a * a + b * b <= (max - a - b).pow(2))
                        .last()
                    {
                        b
                    } else {
                        return false;
                    };

                    a * a + b * b == (max - a - b).pow(2)
                })
                .count()
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
