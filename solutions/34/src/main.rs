use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result: u32 = (3..1_000_000)
        .filter(|number| {
            number
                .to_string()
                .into_bytes()
                .into_iter()
                .map(|ch| {
                    (1..=(ch as char).to_digit(10).unwrap()).product::<u32>()
                })
                .sum::<u32>()
                == *number
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
