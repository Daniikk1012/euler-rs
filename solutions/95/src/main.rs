use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const MAX: usize = 1_000_000;

    let result = (1..=MAX)
        .max_by_key(|&(mut number)| {
            let mut vec = Vec::new();

            while !vec.contains(&number) {
                vec.push(number);

                number = (2..)
                    .take_while(|&divisor: &usize| divisor.pow(2) <= number)
                    .filter(|&divisor| number % divisor == 0)
                    .map(|divisor| {
                        if divisor * divisor == number {
                            divisor
                        } else {
                            divisor + number / divisor
                        }
                    })
                    .sum::<usize>()
                    + 1;

                if number > MAX {
                    return (0, 0);
                }
            }

            if vec[0] == number {
                (vec.len(), usize::MAX - vec[0])
            } else {
                (0, 0)
            }
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
