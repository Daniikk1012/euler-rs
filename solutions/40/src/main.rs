use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut result = 1;

    let mut index = 0;

    for number in 1.. {
        let mut mask = 1;

        while mask <= number {
            mask *= 10;
        }

        mask /= 10;

        while mask > 0 {
            index += 1;

            if [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000].contains(&index)
            {
                result *= number / mask % 10;
            }

            mask /= 10;
        }

        if index > 1_000_000 {
            break;
        }
    }

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
