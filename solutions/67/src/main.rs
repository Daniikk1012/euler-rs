use std::{fs, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let mut triangle: Vec<Vec<u32>> = fs::read_to_string("inputs/67.txt")
        .unwrap()
        .lines()
        .map(|line| line.split(' ').map(|word| word.parse().unwrap()).collect())
        .collect();

    for row in (0..triangle.len() - 1).rev() {
        for col in 0..triangle[row].len() {
            triangle[row][col] +=
                triangle[row + 1][col].max(triangle[row + 1][col + 1]);
        }
    }

    let result = triangle[0][0];

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
