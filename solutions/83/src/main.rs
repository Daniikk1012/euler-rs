use std::{fs, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let matrix: Vec<Vec<u32>> = fs::read_to_string("inputs/83.txt")
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|word| word.parse().unwrap()).collect())
        .collect();

    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    let mut path = vec![vec![u32::MAX; matrix[0].len()]; matrix.len()];
    path[0][0] = matrix[0][0];

    let mut to_visit: Vec<_> = (0..matrix.len())
        .flat_map(|row| (0..matrix[row].len()).map(move |col| [row, col]))
        .collect();

    let result = loop {
        let (index, &[row, col]) = to_visit
            .iter()
            .enumerate()
            .min_by_key(|(_, &[row, col])| path[row][col])
            .unwrap();
        to_visit.remove(index);

        if row == matrix.len() - 1 && col == matrix[row].len() - 1 {
            break path[row][col];
        }

        if row > 0 && !visited[row - 1][col] {
            path[row - 1][col] =
                path[row - 1][col].min(path[row][col] + matrix[row - 1][col]);
        }

        if col > 0 && !visited[row][col - 1] {
            path[row][col - 1] =
                path[row][col - 1].min(path[row][col] + matrix[row][col - 1]);
        }

        if row < matrix.len() - 1 && !visited[row + 1][col] {
            path[row + 1][col] =
                path[row + 1][col].min(path[row][col] + matrix[row + 1][col]);
        }

        if col < matrix[0].len() - 1 && !visited[row][col + 1] {
            path[row][col + 1] =
                path[row][col + 1].min(path[row][col] + matrix[row][col + 1]);
        }

        visited[row][col] = true;
    };

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
