use std::time::SystemTime;

fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a < b {
        (a, b) = (b, a);
    }

    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn main() {
    let time = SystemTime::now();

    let (a, b) = (10..100)
        .flat_map(|a| {
            (a..100)
                .map(move |b| (a, b))
                .filter(|&(a, b)| a != b && a % 10 != 0 && b % 10 != 0)
                .filter(|&(a, b)| {
                    let d = gcd(a, b);

                    if a % 10 == b % 10 {
                        let ds = gcd(a / 10, b / 10);

                        (a / d, b / d) == (a / 10 / ds, b / 10 / ds)
                    } else if a % 10 == b / 10 {
                        let ds = gcd(a / 10, b % 10);

                        (a / d, b / d) == (a / 10 / ds, b % 10 / ds)
                    } else if a / 10 == b % 10 {
                        let ds = gcd(a % 10, b / 10);

                        (a / d, b / d) == (a % 10 / ds, b / 10 / ds)
                    } else if a / 10 == b / 10 {
                        let ds = gcd(a % 10, b % 10);

                        (a / d, b / d) == (a % 10 / ds, b % 10 / ds)
                    } else {
                        false
                    }
                })
        })
        .fold((1, 1), |(a_1, b_1), (a_2, b_2)| (a_1 * a_2, b_1 * b_2));

    let result = b / gcd(a, b);

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
