use std::time::SystemTime;

fn sqrt(n: u64) -> u64 {
    let mut l = 0;
    let mut r = (1 << 32) - 1;

    while (r - l) > 1 {
        let m = (l + r) / 2;

        if m * m > n {
            r = m;
        } else {
            l = m;
        }
    }

    l
}

fn main() {
    let time = SystemTime::now();

    let result: u64 = (5..=1_000_000_000)
        .filter(|&p| {
            let (c, a): (u64, u64) = if p % 3 == 1 {
                (p / 3 * 2, p / 3 + 1)
            } else if p % 3 == 2 {
                ((p / 3 + 1) * 2, p / 3)
            } else {
                return false;
            };

            let b2 = c.pow(2) - a.pow(2);

            sqrt(b2).pow(2) == b2
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
