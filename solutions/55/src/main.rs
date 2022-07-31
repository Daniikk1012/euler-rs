use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result = (1..10_000)
        .filter(|&(mut number): &u128| {
            for _ in 0..50 {
                let mut right = 1;

                let mut reversed = 0;

                while right <= number {
                    reversed *= 10;
                    reversed += number / right % 10;
                    right *= 10;
                }

                number += reversed;

                let mut left = 1;

                while left <= number {
                    left *= 10;
                }

                left /= 10;
                right = 1;

                let mut is_palindrome = true;

                while left > right {
                    if number / left % 10 != number / right % 10 {
                        is_palindrome = false;
                    }

                    left /= 10;
                    right *= 10;
                }

                if is_palindrome {
                    return false;
                }
            }

            true
        })
        .count();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
