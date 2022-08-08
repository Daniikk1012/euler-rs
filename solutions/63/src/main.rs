use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let result = (1..22)
        .flat_map(|power| {
            (1..10)
                .map(move |base: u128| base.pow(power))
                .skip_while(move |&number| number < 10u128.pow(power - 1))
        })
        .count();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
