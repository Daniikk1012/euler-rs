use std::{collections::HashMap, time::SystemTime};

struct CachedCollatz(HashMap<u32, u32>);

impl CachedCollatz {
    fn new() -> Self {
        CachedCollatz(HashMap::new())
    }

    fn get(&mut self, number: u32) -> u32 {
        if let Some(value) = self.0.get(&number) {
            *value
        } else if number == 1 {
            self.0.insert(number, 1);
            1
        } else {
            let result = if number % 2 == 0 {
                self.get(number / 2) + 1
            } else {
                self.get(number * 3 + 1) + 1
            };
            self.0.insert(number, result);
            result
        }
    }
}

fn main() {
    let time = SystemTime::now();

    let mut cached_collatz = CachedCollatz::new();

    let result = (1..1_000_000)
        .max_by_key(|number| cached_collatz.get(*number))
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
