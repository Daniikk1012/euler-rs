use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result: u32 = (2..1_000_000)
        .filter(|number| {
            *number
                == number
                    .to_string()
                    .into_bytes()
                    .into_iter()
                    .map(|ch| (ch as char).to_digit(10).unwrap().pow(5))
                    .sum()
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
