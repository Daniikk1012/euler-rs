use std::time::SystemTime;

fn min_distance(p: &[[u64; 2]]) -> u64 {
    match p.len() {
        0..=1 => u64::MAX,
        2 => {
            p[0][0].abs_diff(p[1][0]).pow(2) + p[0][1].abs_diff(p[1][1]).pow(2)
        }
        _ => {
            let mid = p.len() / 2;
            let (left, right) =
                (min_distance(&p[..mid]), min_distance(&p[mid..]));
            left.min(right).min(
                p[mid - 1][0].abs_diff(p[mid][0]).pow(2)
                    + p[mid - 1][1].abs_diff(p[mid][1]).pow(2),
            )
        }
    }
}

fn main() {
    let time = SystemTime::now();

    let s = {
        let mut s = vec![0u64; 4_000_000];
        s[0] = 290_797;

        for i in 1..s.len() {
            s[i] = s[i - 1].pow(2) % 50_515_093;
        }

        s
    };

    let p = {
        let mut p: Vec<_> =
            (0..2_000_000).map(|i| [s[i * 2], s[i * 2 + 1]]).collect();
        p.sort_unstable_by_key(|&[x, _]| x);
        p
    };

    let result = format!("{:.9}", (min_distance(&p) as f64).sqrt());

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
