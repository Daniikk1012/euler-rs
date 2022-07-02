use std::time::SystemTime;

use primter::Primes;

fn main() {
    let time = SystemTime::now();

    let result = Primes::new()
        .into_iter()
        .skip_while(|prime| *prime <= 5)
        .take_while(|prime| *prime < 1_000)
        .max_by_key(|divisor| {
            let mut count = 1;
            let mut divided = 10 % divisor;

            while divided != 1 {
                divided *= 10;
                divided %= divisor;
                count += 1;
            }

            count
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
