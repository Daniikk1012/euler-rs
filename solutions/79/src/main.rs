use std::{fs, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let subsequences: Vec<u64> = fs::read_to_string("inputs/79.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let result: u64 = (10_000_000..)
        .find(|&password| {
            subsequences.iter().all(|&subsequence| {
                let mut password_mask = 1;
                let mut subsequence_mask = 1;

                while password_mask <= password
                    && subsequence_mask <= subsequence
                {
                    if password / password_mask % 10
                        == subsequence / subsequence_mask % 10
                    {
                        subsequence_mask *= 10;
                    }

                    password_mask *= 10;
                }

                subsequence_mask > subsequence
            })
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
