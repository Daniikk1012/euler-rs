use std::{fs, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let mut matrix: Vec<Vec<u32>> = fs::read_to_string("inputs/81.txt")
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|word| word.parse().unwrap()).collect())
        .collect();

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if row > 0 && col > 0 {
                matrix[row][col] +=
                    matrix[row][col - 1].min(matrix[row - 1][col]);
            } else if row > 0 {
                matrix[row][col] += matrix[row - 1][col]
            } else if col > 0 {
                matrix[row][col] += matrix[row][col - 1]
            }
        }
    }

    let result = matrix.last().unwrap().last().unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
