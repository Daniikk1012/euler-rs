use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result = (1..10_000)
        .filter_map(|number| {
            let mut digits = [false; 10];
            digits[0] = true;

            let mut result = 0;
            let mut result_mask = 1;

            for factor in 1.. {
                let product = number * factor;

                let mut mask = 1;

                while mask <= product {
                    mask *= 10;
                }

                result *= mask;
                result += product;
                result_mask *= mask;

                mask /= 10;

                while mask > 0 {
                    if !digits[product / mask % 10] {
                        digits[product / mask % 10] = true;
                    } else {
                        return None;
                    }

                    mask /= 10;
                }

                if result_mask == 1_000_000_000 {
                    break;
                }
            }

            Some(result)
        })
        .max()
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
