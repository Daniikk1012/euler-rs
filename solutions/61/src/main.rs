use std::time::SystemTime;

fn calculate(begin: u32, mask: [bool; 6], last: u32) -> Option<u32> {
    if mask.iter().filter(|&&flag| flag).count() == 5 {
        let number = begin * 100 + last;

        return if !mask[0] && {
            let n = (((1.0 + number as f32 * 8.0).sqrt() - 1.0) / 2.0) as u32;
            n * (n + 1) / 2 == number
        } || !mask[1] && {
            let n = (number as f32).sqrt() as u32;
            n * n == number
        } || !mask[2] && {
            let n = (((1.0 + number as f32 * 24.0).sqrt() + 1.0) / 6.0) as u32;
            n * (n * 3 - 1) / 2 == number
        } || !mask[3] && {
            let n = (((1.0 + number as f32 * 8.0).sqrt() + 1.0) / 4.0) as u32;
            n * (n * 2 - 1) == number
        } || !mask[4] && {
            let n = (((9.0 + number as f32 * 40.0).sqrt() + 3.0) / 10.0) as u32;
            n * (n * 5 - 3) / 2 == number
        } || !mask[5] && {
            let n = (((1.0 + number as f32 * 3.0).sqrt() + 1.0) / 3.0) as u32;
            n * (n * 3 - 2) == number
        } {
            Some(number)
        } else {
            None
        };
    }

    (10..100).find_map(|end| {
        let number = begin * 100 + end;

        let result = if !mask[0] && {
            let n = (((1.0 + number as f32 * 8.0).sqrt() - 1.0) / 2.0) as u32;
            n * (n + 1) / 2 == number
        } {
            let mut mask = mask;
            mask[0] = true;
            calculate(end, mask, last).map(|sum| sum + number)
        } else {
            None
        };

        if result.is_some() {
            return result;
        }

        let result = if !mask[1] && {
            let n = (number as f32).sqrt() as u32;
            n * n == number
        } {
            let mut mask = mask;
            mask[1] = true;
            calculate(end, mask, last).map(|sum| sum + number)
        } else {
            None
        };

        if result.is_some() {
            return result;
        }

        let result = if !mask[2] && {
            let n = (((1.0 + number as f32 * 24.0).sqrt() + 1.0) / 6.0) as u32;
            n * (n * 3 - 1) / 2 == number
        } {
            let mut mask = mask;
            mask[2] = true;
            calculate(end, mask, last).map(|sum| sum + number)
        } else {
            None
        };

        if result.is_some() {
            return result;
        }

        let result = if !mask[3] && {
            let n = (((1.0 + number as f32 * 8.0).sqrt() + 1.0) / 4.0) as u32;
            n * (n * 2 - 1) == number
        } {
            let mut mask = mask;
            mask[3] = true;
            calculate(end, mask, last).map(|sum| sum + number)
        } else {
            None
        };

        if result.is_some() {
            return result;
        }

        let result = if !mask[4] && {
            let n = (((9.0 + number as f32 * 40.0).sqrt() + 3.0) / 10.0) as u32;
            n * (n * 5 - 3) / 2 == number
        } {
            let mut mask = mask;
            mask[4] = true;
            calculate(end, mask, last).map(|sum| sum + number)
        } else {
            None
        };

        if result.is_some() {
            return result;
        }

        let result = if !mask[5] && {
            let n = (((1.0 + number as f32 * 3.0).sqrt() + 1.0) / 3.0) as u32;
            n * (n * 3 - 2) == number
        } {
            let mut mask = mask;
            mask[5] = true;
            calculate(end, mask, last).map(|sum| sum + number)
        } else {
            None
        };

        if result.is_some() {
            return result;
        }

        None
    })
}

fn main() {
    let time = SystemTime::now();

    let result =
        (10..100).find_map(|end| calculate(end, [false; 6], end)).unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
