use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result = (144..)
        .find_map(|n| {
            let hexagonal = n * (2 * n - 1);
            let pn =
                ((1.0 + (1.0 + 24.0 * hexagonal as f32).sqrt()) / 6.0) as u32;

            if pn * (3 * pn - 1) / 2 == hexagonal {
                Some(hexagonal)
            } else {
                None
            }
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
