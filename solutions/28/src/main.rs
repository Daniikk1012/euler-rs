use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const SIZE: usize = 1001;

    let result = 1
        + (3..)
            .step_by(2)
            .take(SIZE / 2)
            .map(|number| number * number * 4 - 6 * (number - 1))
            .sum::<u32>();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
