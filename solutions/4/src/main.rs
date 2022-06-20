use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut result: u32 = 0;

    for number_a in 100..1_000 {
        for number_b in number_a..1_000 {
            let str_a = (number_a * number_b).to_string().into_bytes();
            let mut str_b = str_a.clone();
            str_b.reverse();

            if str_a == str_b {
                result = result.max(number_a * number_b);
            }
        }
    }

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
