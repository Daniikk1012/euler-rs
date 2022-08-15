# Project Euler solutions in Rust

This is the collection of all the [Project Euler] solutions I wrote in Rust that
I started after I realized that usually posts on [Project Euler] are not
apparently permanent. The solutions are contained in the [`solutions/`]
directory of this repo, each solution being a member of the cargo workspace with
its own `Cargo.toml`.

## How to run

In order to run the solutions, use `cargo run -p problem-<n>` (Or
`cargo run --release -p problem-<n>` for release build), where `<n>` is the
number of the problem.

## Used crates

I used:

* [`primter`]: for a prime number iterator and a fast way to generate and check
primes. This crate is developed by me due to lack of alternatives, and is no
more used for solutions, because a better alternative existed. It will still be
used in the solutions that used it before, because I don't want to modify the
code that I submitted here as a solution.
* [`num`]: for additional numeral types like `BigInt`, `Ratio`, and `Complex`.
* [`primal`]: for a prime number iterator and a fast way to generate and check
primes. This is crate currently is used instead of the [`primter`] crate.

## Benchmarks

I record the times it takes to find the answer for each of the solutions in the
[`BENCHMARKS.md`] file.

[Project Euler]: https://projecteuler.net
[`solutions/`]: https://github.com/Daniikk1012/euler-rs/tree/master/solutions
[`primter`]: https://github.com/Daniikk1012/primter-rs
[`num`]: https://github.com/rust-num/num
[`primal`]: https://github.com/huonw/primal
[`BENCHMARKS.md`]: https://github.com/Daniikk1012/euler-rs/blob/master/BENCHMARKS.md
