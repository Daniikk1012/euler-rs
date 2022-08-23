use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const MOD: i32 = 1_000_000;

    let mut partitions = vec![1];

    let result = (1..)
        .find(|&number| {
            let value = (1..)
                .zip(
                    (1..)
                        .flat_map(|index| [index, -index])
                        .map(|index| index * (index * 3 - 1) / 2),
                )
                .take_while(|&(_, pentagonal)| pentagonal <= number)
                .map(|(index, pentagonal)| {
                    (-1i32).pow((index + 1) / 2 + 1)
                        * partitions[(number - pentagonal) as usize] as i32
                })
                .fold(0, |result, value| (result + value).rem_euclid(MOD));
            partitions.push(value);
            value % MOD == 0
        })
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
