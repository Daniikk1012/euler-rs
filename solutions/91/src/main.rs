use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const MAX: i32 = 50;

    let result: usize = (0..=MAX)
        .map(|x1| {
            (0..=MAX)
                .map(|y1| {
                    (0..=MAX)
                        .map(|x2| {
                            (0..=MAX)
                                .filter(|&y2| y1 * x2 > x1 * y2)
                                .filter(|&y2| {
                                    let a2 = x1.pow(2) + y1.pow(2);
                                    let b2 = x2.pow(2) + y2.pow(2);
                                    let c2 =
                                        (x1 - x2).pow(2) + (y1 - y2).pow(2);
                                    a2 + b2 == c2
                                        || a2 + c2 == b2
                                        || b2 + c2 == a2
                                })
                                .count()
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
