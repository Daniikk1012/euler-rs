use std::{fs, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let content = fs::read_to_string("inputs/22.txt").unwrap();

    let result: u64 = {
        let mut vec: Vec<_> =
            content.split(',').map(|name| &name[1..name.len() - 1]).collect();
        vec.sort_unstable();
        vec
    }
    .into_iter()
    .enumerate()
    .map(|(index, name)| {
        name.bytes().map(|ch| (ch - b'A' + 1) as u64).sum::<u64>()
            * (index as u64 + 1)
    })
    .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
