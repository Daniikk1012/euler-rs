use std::{collections::HashSet, time::SystemTime};

fn to_pentagonal(n: usize) -> u32 {
    n as u32 * (n as u32 * 3 - 1) / 2
}

fn main() {
    let time = SystemTime::now();

    let mut pentagonals = HashSet::new();
    let mut max = 0;

    let result = (1..)
        .find_map(|right_n| {
            let right = to_pentagonal(right_n);
            (1..right_n).rev().find_map(|left_n| {
                let left = to_pentagonal(left_n);

                while max < left + right {
                    let result = to_pentagonal(pentagonals.len() + 1);
                    pentagonals.insert(result);
                    max = result;
                }
                if pentagonals.contains(&(right - left))
                    && pentagonals.contains(&(right + left))
                {
                    Some(right - left)
                } else {
                    None
                }
            })
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
