use std::{array, time::SystemTime};

fn calculate(vec: &mut Vec<u8>, mask: [bool; 10]) -> Option<[u8; 10]> {
    if vec.len() == 10 {
        if vec[8] + vec[9] == vec[0] + vec[3] {
            Some(array::from_fn(|index| vec[index]))
        } else {
            None
        }
    } else {
        (*vec.first().unwrap_or(&0)..10)
            .rev()
            .filter(|&edge| mask[edge as usize])
            .find_map(|edge| {
                vec.push(edge);

                let mut mask = mask;
                mask[edge as usize] = false;

                let result = (0..9)
                    .rev()
                    .filter(|&inner| mask[inner as usize])
                    .find_map(|inner| {
                        vec.push(inner);

                        let mut mask = mask;
                        mask[inner as usize] = false;

                        let result = if vec.len() < 6
                            || vec[vec.len() - 6] + vec[vec.len() - 5]
                                == vec[vec.len() - 4] + vec[vec.len() - 1]
                        {
                            calculate(vec, mask)
                        } else {
                            None
                        };

                        vec.pop();

                        result
                    });

                vec.pop();

                result
            })
    }
}

fn main() {
    let time = SystemTime::now();

    let array = calculate(&mut Vec::new(), [true; 10])
        .unwrap()
        .map(|number| number + 1);

    let result = (0..5)
        .map(|index| {
            format!(
                "{}{}{}",
                array[index * 2 % array.len()],
                array[(index * 2 + 1) % array.len()],
                array[(index * 2 + 3) % array.len()]
            )
        })
        .collect::<String>();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
