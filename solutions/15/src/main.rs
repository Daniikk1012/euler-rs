use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const SIZE: usize = 21;
    let mut routes = [[1u64; SIZE]; SIZE];

    for row_index in 1..SIZE {
        for col_index in 1..SIZE {
            routes[row_index][col_index] = routes[row_index][col_index - 1]
                + routes[row_index - 1][col_index];
        }
    }

    let result = routes[SIZE - 1][SIZE - 1];

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
