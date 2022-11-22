use std::{fs, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let content = fs::read_to_string("inputs/99.txt").unwrap();

    let result = content
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let mut iter = line.split(',').map(str::parse).map(Result::unwrap);
            let a: u32 = iter.next().unwrap();
            let b: u32 = iter.next().unwrap();
            (index, (a as f32).log2() * b as f32)
        })
        .max_by(|(_, a), (_, b)| {
            a.partial_cmp(b).unwrap()
        })
        .map(|(index, _)| index + 1)
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
