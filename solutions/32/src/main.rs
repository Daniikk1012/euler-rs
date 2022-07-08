use std::{cmp::Ordering, collections::HashSet, time::SystemTime};

fn digits(mut n: u32) -> u32 {
    let mut result = 0;

    while n > 0 {
        n /= 10;
        result += 1;
    }

    result
}

fn main() {
    let time = SystemTime::now();

    let set: HashSet<_> = (1..10_000)
        .flat_map(|a| {
            (1..10_000)
                .filter_map(move |b| {
                    let product = a * b;

                    match (digits(a) + digits(b) + digits(product)).cmp(&9) {
                        Ordering::Equal => {
                            let mut digits = [false; 10];
                            digits[0] = true;

                            let mut tmp = a;

                            while tmp > 0 {
                                if digits[tmp as usize % 10] {
                                    return None;
                                } else {
                                    digits[tmp as usize % 10] = true;
                                }

                                tmp /= 10;
                            }

                            tmp = b;

                            while tmp > 0 {
                                if digits[tmp as usize % 10] {
                                    return None;
                                } else {
                                    digits[tmp as usize % 10] = true;
                                }

                                tmp /= 10;
                            }

                            tmp = product;

                            while tmp > 0 {
                                if digits[tmp as usize % 10] {
                                    return None;
                                } else {
                                    digits[tmp as usize % 10] = true;
                                }

                                tmp /= 10;
                            }

                            Some(product)
                        }
                        Ordering::Greater => Some(0),
                        Ordering::Less => None,
                    }
                })
                .take_while(|product| *product > 0)
        })
        .collect();

    let result: u32 = set.into_iter().sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
