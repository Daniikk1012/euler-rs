use std::{collections::HashSet, time::SystemTime};

fn apply_op(op: i32, a: f32, b: f32) -> f32 {
    match op {
        0 => a + b,
        1 => a - b,
        2 => a * b,
        3 => a / b,
        _ => unreachable!(),
    }
}

fn count(a: i32, b: i32, c: i32, d: i32) -> usize {
    let ns = [a, b, c, d];
    let set: HashSet<_> = ns
        .into_iter()
        .flat_map(|n| {
            let a = n;
            ns.into_iter().filter(move |&n| n != a).flat_map(move |n| {
                let b = n;
                ns.into_iter().filter(move |&n| n != a && n != b).flat_map(
                    move |n| {
                        let c = n;
                        ns.into_iter()
                            .filter(move |&n| n != a && n != b && n != c)
                            .flat_map(move |n| {
                                let a = a as f32;
                                let b = b as f32;
                                let c = c as f32;
                                let d = n as f32;
                                (0..4).flat_map(move |oa| {
                                    (0..4).flat_map(move |ob| {
                                        (0..4).flat_map(move |oc| {
                                            [
                                                apply_op(
                                                    oc,
                                                    apply_op(
                                                        ob,
                                                        apply_op(oa, a, b),
                                                        c,
                                                    ),
                                                    d,
                                                ),
                                                apply_op(
                                                    ob,
                                                    apply_op(oa, a, b),
                                                    apply_op(oc, c, d),
                                                ),
                                                apply_op(
                                                    oc,
                                                    apply_op(
                                                        oa,
                                                        a,
                                                        apply_op(ob, b, c),
                                                    ),
                                                    d,
                                                ),
                                                apply_op(
                                                    oa,
                                                    a,
                                                    apply_op(
                                                        oc,
                                                        apply_op(ob, b, c),
                                                        d,
                                                    ),
                                                ),
                                                apply_op(
                                                    oa,
                                                    a,
                                                    apply_op(
                                                        ob,
                                                        b,
                                                        apply_op(oc, c, d),
                                                    ),
                                                ),
                                            ]
                                        })
                                    })
                                })
                            })
                    },
                )
            })
        })
        .filter_map(|n| {
            let i = n.round() as i32;

            if (n - (i as f32)).abs() < f32::EPSILON {
                Some(i)
            } else {
                None
            }
        })
        .filter(|&n| n > 0)
        .collect();
    (1..).take_while(|n| set.contains(n)).count()
}

fn main() {
    let time = SystemTime::now();

    let result = (0..10)
        .flat_map(|d| {
            (0..d).flat_map(move |c| {
                (0..c).flat_map(move |b| {
                    (0..b).map(move |a| ([a, b, c, d], count(a, b, c, d)))
                })
            })
        })
        .max_by_key(|&(_, count)| count)
        .map(|([a, b, c, d], _)| format!("{}{}{}{}", a, b, c, d))
        .unwrap();

    println!("Result: {}", result);
    println!("Time: {}ms", time.elapsed().unwrap().as_millis());
}
