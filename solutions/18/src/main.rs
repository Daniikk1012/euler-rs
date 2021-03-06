use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    #[allow(clippy::zero_prefixed_literal)]
    let mut triangle: [Vec<u32>; 15] = [
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 04, 82, 47, 65],
        vec![19, 01, 23, 75, 03, 34],
        vec![88, 02, 77, 73, 07, 63, 67],
        vec![99, 65, 04, 28, 06, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
    ];

    for row_index in (0..triangle.len() - 1).rev() {
        for col_index in 0..triangle[row_index].len() {
            triangle[row_index][col_index] += triangle[row_index + 1]
                [col_index]
                .max(triangle[row_index + 1][col_index + 1]);
        }
    }

    let result = triangle[0][0];

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
