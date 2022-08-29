use std::{array, convert, time::SystemTime};

use rand::Rng;

fn main() {
    let time = SystemTime::now();

    let mut frequencies = [0; 40];
    let mut current = 0;

    let mut rng = rand::thread_rng();

    for _ in 0..10_000_000 {
        let mut counter = 0;

        while {
            let first = rng.gen_range(1..=4);
            let second = rng.gen_range(1..=4);

            if first == second {
                counter += 1;
            }

            current += first + second;
            current %= 40;

            if let 7 | 22 | 36 = current {
                current = match rng.gen_range(1..=16) {
                    1 => 0,
                    2 => 10,
                    3 => 11,
                    4 => 24,
                    5 => 39,
                    6 => 5,
                    7 | 8 => match current {
                        7 => 15,
                        22 => 25,
                        36 => 5,
                        _ => unreachable!(),
                    },
                    9 => {
                        if let 13..=27 = current {
                            28
                        } else {
                            12
                        }
                    }
                    10 => current - 3,
                    _ => current,
                };
            }

            if let 2 | 17 | 33 = current {
                current = match rng.gen_range(1..=16) {
                    1 => 0,
                    2 => 10,
                    _ => current,
                };
            }

            if current == 30 || counter == 3 {
                current = 10;
            }

            frequencies[current] += 1;

            first == second && current != 10
        } {}
    }

    let mut indicies: [_; 40] = array::from_fn(convert::identity);
    indicies.sort_unstable_by_key(|&index| frequencies[index]);

    let result: String = indicies[indicies.len() - 3..indicies.len()]
        .iter()
        .rev()
        .map(|index| format!("{:02}", index))
        .collect();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
