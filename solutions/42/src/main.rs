use std::{fs, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let result = fs::read_to_string("inputs/42.txt")
        .unwrap()
        .split(',')
        .filter(|word| {
            let score = word.as_bytes()[1..word.len() - 1]
                .iter()
                .map(|&ch| (ch - b'A' + 1) as u32)
                .sum::<u32>();

            let sqrt = (score as f32 * 2.0).sqrt() as u32;

            sqrt * (sqrt + 1) / 2 == score
        })
        .count();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
