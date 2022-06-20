use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut result = 0;

    'root: for a in 1..1_000 {
        for b in a..(1_000 - a) / 2 {
            let c = 1_000 - a - b;

            if a * a + b * b > c * c {
                break;
            } else if a * a + b * b == c * c {
                result = a * b * c;
                break 'root;
            }
        }
    }

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
