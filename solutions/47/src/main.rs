use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut sequence = vec![];

    let result = (1..)
        .find_map(|number| {
            let mut number_mut = number;

            let mut count = 0;

            for divisor in 2.. {
                if divisor * divisor > number_mut {
                    break;
                }

                if number_mut % divisor == 0 {
                    while number_mut % divisor == 0 {
                        number_mut /= divisor;
                    }

                    count += 1;
                }
            }

            if number > 1 {
                count += 1;
            }

            if count == 4 {
                sequence.push(number);
            } else {
                sequence.clear();
            }

            if sequence.len() == 4 {
                Some(sequence[0])
            } else {
                None
            }
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
