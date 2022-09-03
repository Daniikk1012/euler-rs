use std::{fs, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let data = fs::read_to_string("inputs/89.txt").unwrap();

    let result: usize = data
        .lines()
        .map(ToString::to_string)
        .map(|line| {
            line.len()
                - line
                    .replace("DCCCC", "CM")
                    .replace("CCCC", "CD")
                    .replace("LXXXX", "XC")
                    .replace("XXXX", "XL")
                    .replace("VIIII", "IX")
                    .replace("IIII", "IV")
                    .len()
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
