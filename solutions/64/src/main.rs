use std::time::SystemTime;

trait Gcd<Rhs = Self> {
    type Output;

    fn gcd(self, rhs: Rhs) -> Self::Output;
}

impl Gcd for i32 {
    type Output = i32;

    fn gcd(mut self, mut rhs: Self) -> Self::Output {
        self = self.abs();
        rhs = rhs.abs();

        if self < rhs {
            (self, rhs) = (rhs, self);
        }

        while rhs > 0 {
            (self, rhs) = (rhs, self % rhs);
        }

        self
    }
}

fn main() {
    let time = SystemTime::now();

    let result = (1..=10_000)
        .filter(|&number| {
            let sqrt = (number as f32).sqrt();

            if (sqrt as i32).pow(2) == number {
                return false;
            }

            let (mut left, mut right, mut bottom) = (1, -sqrt as i32, 1);

            (1..)
                .find(|_| {
                    let limit = (bottom as f32
                        / (left as f32 * sqrt + right as f32))
                        as i32;

                    (left, right, bottom) =
                        (left * bottom, -right * bottom, number - right.pow(2));
                    right -= limit * bottom;

                    let gcd = left.gcd(right).gcd(bottom);
                    left /= gcd;
                    right /= gcd;
                    bottom /= gcd;

                    bottom == 1
                })
                .unwrap()
                % 2
                == 1
        })
        .count();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
