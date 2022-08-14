use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const MAX: u32 = 1_000_000;

    let result = (1..=MAX)
        .map(|number| {
            (
                number,
                number as f32 / {
                    let mut divisors = Vec::new();
                    let mut number_mut = number;
                    let mut divisor: u32 = 2;

                    while divisor.pow(2) <= number_mut {
                        if number_mut % divisor == 0 {
                            divisors.push((divisor, 0));

                            while number_mut % divisor == 0 {
                                divisors.last_mut().unwrap().1 += 1;
                                number_mut /= divisor;
                            }
                        }

                        divisor += 1;
                    }

                    if number_mut > 1 {
                        divisors.push((number_mut, 1));
                    }

                    divisors
                }
                .into_iter()
                .map(|(prime, count)| prime.pow(count - 1) * (prime - 1))
                .product::<u32>() as f32,
            )
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(number, _)| number)
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
