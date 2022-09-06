use std::time::SystemTime;

fn count(set: &mut Vec<Option<bool>>, number: usize) {
    if set[number].is_none() {
        let mut result = 0;
        let mut number_mut = number;

        while number_mut > 0 {
            result += (number_mut % 10).pow(2);
            number_mut /= 10;
        }

        count(set, result);

        set[number] = Some(set[result].unwrap());
    }
}

fn main() {
    let time = SystemTime::now();

    const MAX: usize = 10_000_000;

    let mut set = vec![None; MAX];
    set[0] = Some(false);
    set[1] = Some(false);
    set[89] = Some(true);

    for number in 1..MAX {
        count(&mut set, number);
    }

    let result = set.into_iter().filter(|&x| x.unwrap()).count();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
