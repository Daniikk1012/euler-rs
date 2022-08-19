use std::{collections::HashMap, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let mut cache = HashMap::new();
    cache.insert(1, 1);
    cache.insert(2, 1);
    cache.insert(145, 1);
    cache.insert(169, 3);
    cache.insert(871, 2);
    cache.insert(872, 2);
    cache.insert(1454, 3);
    cache.insert(40_585, 1);
    cache.insert(45_361, 2);
    cache.insert(45_362, 2);
    cache.insert(363_601, 3);

    let mut vec = Vec::new();

    let result = (1..1_000_000)
        .filter(|&number| {
            vec.clear();

            let mut number_mut = number;

            while !cache.contains_key(&number_mut) {
                vec.push(number_mut);

                let mut result = 0;

                while number_mut > 0 {
                    result += (1..=number_mut % 10).product::<usize>();
                    number_mut /= 10;
                }

                number_mut = result;
            }

            let mut previous = cache[&number_mut];

            for &number in vec.iter().rev() {
                cache.insert(number, previous + 1);
                previous = cache[&number];
            }

            cache[&number] == 60
        })
        .count();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
