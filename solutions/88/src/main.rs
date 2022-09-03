use std::{collections::HashSet, time::SystemTime};

fn main() {
    let time = SystemTime::now();

    const MAX: usize = 12_000;

    let mut results = [usize::MAX; MAX + 1];

    for len in 1..=15 {
        let mut terms = vec![2; len];

        'root: loop {
            let n: usize = terms.iter().product();
            let k = n - terms.iter().sum::<usize>() + len;

            if k <= MAX {
                results[k] = results[k].min(n);
            }

            for index in 0..len {
                if terms[index..].iter().product::<usize>()
                    * (terms[index] + 1).pow(index as u32)
                    <= MAX * 2
                {
                    terms[index] += 1;

                    for index_before in 0..index {
                        terms[index_before] = terms[index];
                    }

                    continue 'root;
                }
            }

            break;
        }
    }

    let result: usize =
        results[2..].iter().collect::<HashSet<_>>().into_iter().sum();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
