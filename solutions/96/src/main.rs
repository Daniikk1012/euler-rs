use std::{array, fs, time::SystemTime};

fn solve(mut sudoku: [[u32; 9]; 9]) -> Option<[[u32; 9]; 9]> {
    if sudoku.into_iter().flatten().any(|digit| digit == 0) {
        (0..9)
            .flat_map(|row| {
                (0..9).filter(move |&col| sudoku[row][col] == 0).map(
                    move |col| {
                        let mut count = 0;

                        let array: [_; 9] = array::from_fn(|index| {
                            let digit = index as u32 + 1;

                            if sudoku[row].iter().all(|&d| d != digit)
                                && (0..9)
                                    .map(|row| sudoku[row][col])
                                    .all(|d| d != digit)
                                && (row - row % 3..row - row % 3 + 3)
                                    .flat_map(|row| {
                                        (col - col % 3..col - col % 3 + 3)
                                            .map(move |col| sudoku[row][col])
                                    })
                                    .all(|d| d != digit)
                            {
                                count += 1;
                                true
                            } else {
                                false
                            }
                        });

                        (count, [row, col], array)
                    },
                )
            })
            .min_by_key(|&(count, _, _)| count)
            .and_then(|(_, [row, col], array)| {
                array
                    .into_iter()
                    .enumerate()
                    .filter(|&(_, value)| value)
                    .map(|(index, _)| index as u32 + 1)
                    .find_map(|digit| {
                        sudoku[row][col] = digit;

                        solve(sudoku)
                    })
            })
    } else {
        Some(sudoku)
    }
}

fn main() {
    let time = SystemTime::now();

    let result: u32 = fs::read_to_string("inputs/96.txt")
        .unwrap()
        .lines()
        .collect::<Vec<_>>()
        .chunks(10)
        .map(|chunk| {
            chunk[1..]
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|ch| ch.to_digit(10).unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .map(solve)
        .map(Option::unwrap)
        .map(|sudoku| sudoku[0][0] * 100 + sudoku[0][1] * 10 + sudoku[0][2])
        .sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
