use std::{collections::HashMap, time::SystemTime};

use num_integer::Integer;

fn main() {
    let time = SystemTime::now();

    const MAX: u64 = 1_500_000;

    let mut map: HashMap<u64, Option<[u64; 3]>> = HashMap::new();

    for m in (1..).take_while(|&m: &u64| m.pow(2) < MAX) {
        for n in (1..m)
            .take_while(|&n: &u64| m.pow(2) + n.pow(2) < MAX)
            .filter(|n| m.gcd(n) == 1)
            .filter(|&n| m % 2 != n % 2)
        {
            for d in (1..).take_while(|&d| 2 * m * (m + n) * d <= MAX) {
                let mut a = (m.pow(2) - n.pow(2)) * d;
                let mut b = 2 * m * n * d;
                let c = (m.pow(2) + n.pow(2)) * d;

                if a > b {
                    (a, b) = (b, a);
                }

                map.entry(a + b + c)
                    .and_modify(|option| {
                        *option = option.map_or(None, |triplet| {
                            if [a, b, c] == triplet {
                                Some(triplet)
                            } else {
                                None
                            }
                        });
                    })
                    .or_insert(Some([a, b, c]));
            }
        }
    }

    let result = map.into_values().filter(Option::is_some).count();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
