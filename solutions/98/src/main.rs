use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs,
    time::SystemTime,
};

fn map_characters(square: &[u8], key: &[u8]) -> Option<HashMap<u8, u8>> {
    let mut used = HashSet::new();

    if square.len() == key.len() {
        let mut map = HashMap::new();

        for (digit, character) in
            square.iter().cloned().zip(key.iter().cloned())
        {
            match map.entry(character) {
                Entry::Vacant(entry) => {
                    if !used.contains(&digit) {
                        entry.insert(digit);
                        used.insert(digit);
                    } else {
                        return None;
                    }
                }
                Entry::Occupied(entry) => {
                    if *entry.get() != digit {
                        return None;
                    }
                }
            }
        }

        Some(map)
    } else {
        None
    }
}

fn apply_map(map: &HashMap<u8, u8>, value: &mut [u8]) {
    for ch in value {
        *ch = map[ch];
    }
}

fn main() {
    let time = SystemTime::now();

    let mut words: HashMap<_, Vec<_>> = HashMap::new();

    for word in fs::read_to_string("inputs/98.txt")
        .unwrap()
        .split(',')
        .map(|word| &word[1..word.len() - 1])
    {
        let mut sorted = word.to_string().into_bytes();
        sorted.sort_unstable();
        words
            .entry(sorted)
            .and_modify(|vec| vec.push(word.to_string().into_bytes()))
            .or_insert_with(|| vec![word.to_string().into_bytes()]);
    }

    words.retain(|_, vec| vec.len() > 1);

    let max = words.keys().map(|key| key.len()).max().unwrap();

    let result: u32 = (1..)
        .map(|n: u32| n.pow(2))
        .map(|n| (n, n.to_string().into_bytes()))
        .take_while(|(_, vec)| vec.len() <= max)
        .filter_map(|(n, square)| {
            words
                .values()
                .filter_map(|vec| {
                    vec.iter()
                        .filter_map(|word| {
                            map_characters(&square, word).and_then(|map| {
                                if vec
                                    .iter()
                                    .filter(|&word| {
                                        let mut copy = word.clone();
                                        apply_map(&map, &mut copy);

                                        if copy[0] != b'0' {
                                            let number =
                                                String::from_utf8(copy)
                                                    .unwrap()
                                                    .parse()
                                                    .unwrap();
                                            ((number as f32).sqrt() as u32)
                                                .pow(2)
                                                == number
                                        } else {
                                            false
                                        }
                                    })
                                    .count()
                                    > 1
                                {
                                    Some(n)
                                } else {
                                    None
                                }
                            })
                        })
                        .max()
                })
                .max()
        })
        .max()
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
