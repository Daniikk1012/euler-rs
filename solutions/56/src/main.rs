use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut vec = Vec::new();

    let result: u32 = (1..100)
        .map(move |a| {
            let vec = &mut vec;

            (1..100)
                .map(move |b| {
                    vec.clear();
                    vec.push(1);

                    for _ in 0..b {
                        let mut remainder = 0;

                        for digit in vec.iter_mut() {
                            *digit *= a;
                            *digit += remainder;
                            remainder = *digit / 10;
                            *digit %= 10;
                        }

                        while remainder > 0 {
                            vec.push(remainder % 10);
                            remainder /= 10;
                        }
                    }

                    vec.iter().sum()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
