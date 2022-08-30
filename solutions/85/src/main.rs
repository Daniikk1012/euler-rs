use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const MAX: u32 = 2_000_000;

    let result = (1..)
        .map(|n| (n, n * (n + 1) / 2))
        .take_while(|&(_, x)| x * x < MAX)
        .map(|(n, x)| {
            let m = (((1.0
                + (16.0 * MAX as f32) / (n as f32 * (n as f32 + 1.0)))
                .sqrt()
                - 1.0)
                / 2.0)
                .round() as u32;
            let y = m * (m + 1) / 2;
            (n * m, x * y)
        })
        .min_by_key(|&(_, r)| r.abs_diff(MAX))
        .map(|(p, _)| p)
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
