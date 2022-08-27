use std::{fs, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    let matrix: Vec<Vec<u32>> = fs::read_to_string("inputs/82.txt")
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|word| word.parse().unwrap()).collect())
        .collect();

    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    let mut path = vec![vec![u32::MAX; matrix[0].len()]; matrix.len()];

    for (path_first, matrix_first) in path
        .iter_mut()
        .map(|row| row.first_mut().unwrap())
        .zip(matrix.iter().map(|row| *row.first().unwrap()))
    {
        *path_first = matrix_first;
    }

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

        if col == matrix[row].len() - 1 {
            break path[row][col];
        }

        if row > 0 && !visited[row - 1][col] {
            path[row - 1][col] =
                path[row - 1][col].min(path[row][col] + matrix[row - 1][col]);
        }

        if row < matrix.len() - 1 && !visited[row + 1][col] {
            path[row + 1][col] =
                path[row + 1][col].min(path[row][col] + matrix[row + 1][col]);
        }

        if col < matrix[row].len() - 1 && !visited[row][col + 1] {
            path[row][col + 1] =
                path[row][col + 1].min(path[row][col] + matrix[row][col + 1]);
        }

        visited[row][col] = true;
    };

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
