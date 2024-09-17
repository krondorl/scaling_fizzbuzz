<img src="docs/logo.svg" width="320"/>

# Scaling FizzBuzz

I wanted to go beyond simple coding, by scaling the FizzBuzz algorithm using Rust.

## Inspiration

I was inspired by the Code Golf competition on [Code Golf Stack Exchange](https://codegolf.stackexchange.com/) called [High Throughput Fizz Buzz](https://codegolf.stackexchange.com/questions/215216/high-throughput-fizz-buzz/).

## Source Code

The code is in the `src/bin` folder.

## Installation

1. Install [the Rust tools](https://www.rust-lang.org/tools/install).
1. Install `hyperfine`, by executing `cargo install hyperfine`

## Running separate binaries

Inside the project folder run this:

```
cargo run --bin 01-fb-for
```

## Running the Benchmark

Go to the repo folder then execute the following:

```
hyperfine 'cargo run --release --bin 01-fb-for' 'cargo run --release --bin 02-fb-match' 'cargo run --release --bin 03-fb-par-iter'
```

This will compare separate programs inside `bin`.

## License

Please see the [LICENSE file](LICENSE).

## History

Started in September, 2024.
