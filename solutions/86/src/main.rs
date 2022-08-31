use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const MIN: usize = 1_000_000;

    let result = (1..)
        .map(|m: u32| {
            (
                m,
                (1..=m)
                    .map(|b| {
                        (1..=b)
                            .filter(|a| {
                                let sum = (a + b).pow(2) + m.pow(2);

                                ((sum as f32).sqrt() as u32).pow(2) == sum
                            })
                            .count()
                    })
                    .sum::<usize>(),
            )
        })
        .scan(0, |state, (m, c)| {
            *state += c;
            Some((m, *state))
        })
        .find(|&(_, c)| c >= MIN)
        .map(|(m, _)| m)
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
