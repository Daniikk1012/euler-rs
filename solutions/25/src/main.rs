use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut a = vec![0];
    let mut b = vec![1];
    let mut t = vec![1];

    let result = (1..).find(|_| {
        for (digit_t, digit_a) in t.iter_mut().zip(&a) {
            *digit_t += digit_a;
        }

        let mut remainder = 0;

        for digit in &mut t {
            *digit += remainder;
            remainder = *digit / 10;
            *digit %= 10;
        }

        while remainder > 0 {
            t.push(remainder % 10);
            remainder /= 10;
        }

        a.clone_from(&b);
        b.clone_from(&t);

        a.len() >= 1_000
    }).unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
