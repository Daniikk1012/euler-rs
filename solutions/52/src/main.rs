use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result = (3..).step_by(3)
        .find(|&number| {
            let mut digits = [0; 10];

            let mut number_mut = number;

            while number_mut > 0 {
                digits[number_mut % 10] += 1;
                number_mut /= 10;
            }

            for mut number in (number..).step_by(number).take(6).skip(1) {
                let mut other_digits = [0; 10];

                while number > 0 {
                    other_digits[number % 10] += 1;
                    number /= 10;
                }

                if other_digits != digits {
                    return false;
                }
            }

            true
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
