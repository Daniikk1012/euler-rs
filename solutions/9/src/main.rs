use std::{cmp::Ordering, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let mut result = 0;

    'root: for a in 1..1_000 {
        for b in a..(1_000 - a) / 2 {
            let c = 1_000 - a - b;

            match (a * a + b * b).cmp(&(c * c)) {
                Ordering::Greater => break,
                Ordering::Equal => {
                    result = a * b * c;
                    break 'root;
                }
                _ => {}
            }
        }
    }

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
