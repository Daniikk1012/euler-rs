use std::time::SystemTime;

fn count(
    [mut left, mut right]: [[bool; 10]; 2],
    [left_amount, right_amount]: [u8; 2],
    index: usize,
) -> usize {
    if left <= right {
        if index == 10 {
            if (left[0] && right[1] || right[0] && left[1])
                && (left[0] && right[4] || right[0] && left[4])
                && (left[0] && (right[6] || right[9])
                    || right[0] && (left[6] || left[9]))
                && (left[1] && (right[6] || right[9])
                    || right[1] && (left[6] || left[9]))
                && (left[2] && right[5] || right[2] && left[5])
                && (left[2] && right[5] || right[2] && left[5])
                && (left[3] && (right[6] || right[9])
                    || right[3] && (left[6] || left[9]))
                && (left[4] && (right[6] || right[9])
                    || right[4] && (left[6] || left[9]))
                && (left[8] && right[1] || right[8] && left[1])
                && left_amount == 0
                && right_amount == 0
            {
                1
            } else {
                0
            }
        } else {
            let mut result = 0;

            result +=
                count([left, right], [left_amount, right_amount], index + 1);

            if right_amount > 0 {
                right[index] = true;
                result += count(
                    [left, right],
                    [left_amount, right_amount - 1],
                    index + 1,
                );
                right[index] = false;
            }

            if left_amount > 0 {
                left[index] = true;
                result += count(
                    [left, right],
                    [left_amount - 1, right_amount],
                    index + 1,
                );

                if right_amount > 0 {
                    right[index] = true;
                    result += count(
                        [left, right],
                        [left_amount - 1, right_amount - 1],
                        index + 1,
                    );
                }
            }

            result
        }
    } else {
        0
    }
}

fn main() {
    let time = SystemTime::now();

    let result = count([[false; 10]; 2], [6; 2], 0);

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
