use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const FACTORIALS: [u32; 10] =
        [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];

    let mut arr = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    let mut order = 999_999;

    while order > 0 {
        let (index, factorial) = FACTORIALS
            .iter()
            .enumerate()
            .rev()
            .find(|(_, factorial)| order >= **factorial)
            .unwrap();

        let to_swap = arr[..index]
            .binary_search_by(|number| arr[index].cmp(number))
            .unwrap_or_else(|index| index - 1);

        (arr[index], arr[to_swap]) = (arr[to_swap], arr[index]);

        order -= factorial;
    }

    let result: String = arr
        .iter()
        .rev()
        .map(|digit| char::from_digit(*digit, 10).unwrap())
        .collect();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
