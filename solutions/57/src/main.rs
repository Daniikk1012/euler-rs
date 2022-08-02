use std::{iter, time::SystemTime};

fn vec_add(left: &mut Vec<u8>, right: &Vec<u8>) {
    let mut remainder = 0;

    while left.len() < right.len() {
        left.push(0);
    }

    for (left_digit, right_digit) in
        left.iter_mut().zip(right.iter().chain(iter::repeat(&0)))
    {
        *left_digit += right_digit;
        *left_digit += remainder;
        remainder = *left_digit / 10;
        *left_digit %= 10;
    }

    while remainder > 0 {
        left.push(remainder % 10);
        remainder /= 10;
    }
}

fn main() {
    let time = SystemTime::now();

    let mut result = 0;

    let (mut vec_a, mut vec_b) = (vec![2], vec![1]);

    for _ in 0..1_000 {
        (vec_a, vec_b) = (vec_b, vec_a);
        vec_add(&mut vec_a, &vec_b);

        if vec_a.len() > vec_b.len() {
            result += 1;
        }

        vec_add(&mut vec_a, &vec_b);
    }

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
