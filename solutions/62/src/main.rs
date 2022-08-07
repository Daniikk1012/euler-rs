use std::{collections::HashMap, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let mut counts: HashMap<usize, HashMap<u64, (_, usize)>> = HashMap::new();

    for cube in (1000..10_000).map(|n: u64| n.pow(3)) {
        let mut digits = cube.to_string().into_bytes();
        digits.sort_unstable();
        let len = digits.len();
        let sorted = String::from_utf8(digits).unwrap().parse().unwrap();

        counts
            .entry(len)
            .and_modify(|map| {
                map.entry(sorted)
                    .and_modify(|(min, count)| {
                        *min = cube.min(*min);
                        *count += 1;
                    })
                    .or_insert((cube, 1));
            })
            .or_insert({
                let mut map = HashMap::new();
                map.entry(sorted)
                    .and_modify(|(min, count)| {
                        *min = cube.min(*min);
                        *count += 1;
                    })
                    .or_insert((cube, 1));
                map
            });
    }

    let result = counts
        .into_values()
        .flat_map(|map| map.into_values())
        .filter_map(|(min, count)| if count == 5 { Some(min) } else { None })
        .min()
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
