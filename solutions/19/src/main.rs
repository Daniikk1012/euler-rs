use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    let mut result = 0;

    let mut year = 1_900;
    let mut month = 1;
    let mut week_day = 0;

    while year <= 2_000 {
        if year >= 1_901 && week_day == 6 {
            result += 1;
        }

        week_day += match month {
            2 => {
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            4 | 6 | 9 | 11 => 30,
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => unreachable!(),
        };

        week_day %= 7;

        month += 1;

        if month == 13 {
            year += 1;
            month = 1;
        }
    }

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
