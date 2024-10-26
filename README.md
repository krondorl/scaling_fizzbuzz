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

## Running Separate Binaries

Inside the project folder run this:

```
cargo run --bin 01-fb-for
```

## Running the Benchmark

Go to the repo folder then execute the following:

```
hyperfine --warmup 1 'cargo run --release --bin 01-fb-for' 'cargo run --release --bin 02-fb-match' 'cargo run --release --bin 03-fb-par-iter' 'cargo run --release --bin 04-fb-par-iter-match' 'cargo run --release --bin 05-fb-par-iter-match-buf'
```

This will compare all programs inside `bin`.

![Console Benchmark Results](/docs/benchmark.png "Console Benchmark Results")

## Configuration

You can set the maximum iteration number in the `config.json` file.

```
{
    "max_iter": 1000
}
```

## License

Please see the [LICENSE file](LICENSE).

## History

Started in September, 2024.
