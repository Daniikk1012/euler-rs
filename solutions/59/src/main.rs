use std::{fs, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let input: Vec<u8> = fs::read_to_string("inputs/59.txt")
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let result: u32 = (b'a'..=b'z')
        .flat_map(|a| (b'a'..=b'z').map(move |b| [a, b]))
        .flat_map(|[a, b]| (b'a'..=b'z').map(move |c| [a, b, c]))
        .find_map(|key| {
            let iter = input
                .iter()
                .zip(key.into_iter().cycle())
                .map(|(&byte, key)| byte ^ key);

            let text = if iter.clone().all(|byte| byte.is_ascii()) {
                iter.map(|byte| byte as char).collect::<String>()
            } else {
                return None;
            };

            if text.contains(" the ") {
                Some(
                    text.into_bytes().into_iter().map(|byte| byte as u32).sum(),
                )
            } else {
                None
            }
        })
        .unwrap();

    println!("Result: {:?}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
