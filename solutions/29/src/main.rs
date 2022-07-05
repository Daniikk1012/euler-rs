use std::{collections::HashSet, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    const MAX: u32 = 100;

    let mut bases = HashSet::new();

    let result: usize = (2..=MAX)
        .map(|a| {
            if bases.contains(&a) {
                return 0;
            }

            let mut powers: HashSet<u32> = HashSet::new();

            let mut base = a;

            for power in 1.. {
                if base > MAX {
                    break;
                }

                bases.insert(base);

                for b in 2..=MAX {
                    powers.insert(b * power);
                }

                base *= a;
            }

            powers.len()
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
