use std::time::SystemTime;

fn count_combinations(coins: &[u32], needed: u32) -> u32 {
    if coins.len() > 1 && needed > 0 {
        let mut count = 0;

        for (index, coin) in
            coins.iter().enumerate().skip_while(|(_, coin)| needed < **coin)
        {
            count += count_combinations(&coins[index..], needed - coin);
        }

        count
    } else {
        1
    }
}

fn main() {
    let time = SystemTime::now();

    let result = count_combinations(&[200, 100, 50, 20, 10, 5, 2, 1], 200);

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
